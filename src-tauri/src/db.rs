use crate::error::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
pub struct ClipboardEntry {
    pub id: i64,
    pub content: String,
    pub created_at: String,
    pub last_used_at: String,
    pub pinned: bool,
    pub label: String,
}

#[derive(Debug, PartialEq)]
pub enum InsertResult {
    New,
    Duplicate,
}

pub struct Database {
    conn: Mutex<Connection>,
}

pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

impl Database {
    pub fn new(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA cache_size=-16000;
             PRAGMA temp_store=MEMORY;
             PRAGMA busy_timeout=5000;
             PRAGMA mmap_size=268435456;",
        )?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    #[cfg(test)]
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clipboard_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                content_hash TEXT UNIQUE NOT NULL,
                created_at TEXT NOT NULL,
                last_used_at TEXT NOT NULL,
                pinned INTEGER NOT NULL DEFAULT 0,
                label TEXT NOT NULL DEFAULT ''
            );
            CREATE INDEX IF NOT EXISTS idx_last_used ON clipboard_entries(last_used_at DESC);",
        )?;
        Ok(())
    }

    pub fn save_entry(&self, content: &str) -> Result<InsertResult> {
        let conn = self.conn.lock()?;
        let hash = hash_content(content);
        let now = chrono::Utc::now().to_rfc3339();

        let last_rowid = conn.last_insert_rowid();
        let mut stmt = conn.prepare_cached(
            "INSERT INTO clipboard_entries (content, content_hash, created_at, last_used_at)
             VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(content_hash) DO UPDATE SET last_used_at = excluded.last_used_at",
        )?;
        stmt.execute(params![content, hash, now])?;

        let result = if conn.last_insert_rowid() != last_rowid {
            InsertResult::New
        } else {
            InsertResult::Duplicate
        };
        Ok(result)
    }

    pub fn list_entries(&self, limit: usize) -> Result<Vec<ClipboardEntry>> {
        let conn = self.conn.lock()?;
        let mut stmt = conn.prepare_cached(
            "SELECT id, content, created_at, last_used_at, pinned, label
             FROM clipboard_entries ORDER BY pinned DESC, last_used_at DESC LIMIT ?1",
        )?;

        let entries = stmt
            .query_map(params![limit], |row| {
                Ok(ClipboardEntry {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    created_at: row.get(2)?,
                    last_used_at: row.get(3)?,
                    pinned: row.get::<_, i32>(4)? != 0,
                    label: row.get(5)?,
                })
            })?
            .filter_map(|e| e.ok())
            .collect();

        Ok(entries)
    }

    pub fn delete_entry(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock()?;
        conn.execute(
            "DELETE FROM clipboard_entries WHERE id = ?1 AND pinned = 0",
            params![id],
        )?;
        Ok(())
    }

    pub fn enforce_history_limit(&self, max_history: usize) -> Result<()> {
        let conn = self.conn.lock()?;
        conn.execute(
            "DELETE FROM clipboard_entries WHERE pinned = 0 AND id NOT IN (
                SELECT id FROM clipboard_entries WHERE pinned = 0 ORDER BY last_used_at DESC LIMIT ?1
            )",
            params![max_history],
        )?;
        Ok(())
    }

    pub fn toggle_pin(&self, id: i64, label: String) -> Result<bool> {
        let conn = self.conn.lock()?;
        let pinned: Option<bool> = conn.query_row(
            "UPDATE clipboard_entries SET pinned = 1 - pinned, label = CASE WHEN pinned = 0 THEN ?1 ELSE '' END WHERE id = ?2 RETURNING pinned",
            params![label, id],
            |row| Ok(row.get::<_, i32>(0)? != 0),
        ).optional()?;
        pinned.ok_or(crate::error::FclipError::NotFound(id))
    }

    pub fn use_entry(&self, id: i64) -> Result<String> {
        let conn = self.conn.lock()?;
        let now = chrono::Utc::now().to_rfc3339();

        let content: Option<String> = conn
            .query_row(
                "UPDATE clipboard_entries SET last_used_at = ?1 WHERE id = ?2 RETURNING content",
                params![now, id],
                |row| row.get(0),
            )
            .optional()?;

        content.ok_or(crate::error::FclipError::NotFound(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_list_entries() {
        let db = Database::open_in_memory().unwrap();

        assert_eq!(db.save_entry("hello").unwrap(), InsertResult::New);
        assert_eq!(db.save_entry("world").unwrap(), InsertResult::New);
        assert_eq!(db.save_entry("hello").unwrap(), InsertResult::Duplicate);

        let entries = db.list_entries(1000).unwrap();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_delete_entry() {
        let db = Database::open_in_memory().unwrap();
        db.save_entry("test").unwrap();

        let entries = db.list_entries(1000).unwrap();
        let id = entries[0].id;

        db.delete_entry(id).unwrap();
        assert_eq!(db.list_entries(1000).unwrap().len(), 0);
    }

    #[test]
    fn test_delete_pinned_entry_is_blocked() {
        let db = Database::open_in_memory().unwrap();
        db.save_entry("pinned item").unwrap();

        let entries = db.list_entries(1000).unwrap();
        let id = entries[0].id;

        db.toggle_pin(id, "keep".to_string()).unwrap();
        db.delete_entry(id).unwrap();
        assert_eq!(
            db.list_entries(1000).unwrap().len(),
            1,
            "pinned entry should not be deleted"
        );
    }

    #[test]
    fn test_enforce_history_limit() {
        let db = Database::open_in_memory().unwrap();
        for i in 0..5 {
            db.save_entry(&format!("entry {}", i)).unwrap();
        }

        db.enforce_history_limit(3).unwrap();
        assert_eq!(db.list_entries(1000).unwrap().len(), 3);
    }

    #[test]
    fn test_use_entry() {
        let db = Database::open_in_memory().unwrap();
        db.save_entry("content here").unwrap();

        let entries = db.list_entries(1000).unwrap();
        let id = entries[0].id;

        let content = db.use_entry(id).unwrap();
        assert_eq!(content, "content here");
    }

    #[test]
    fn test_emoji_round_trip() {
        let db = Database::open_in_memory().unwrap();
        let emoji_text = "Hello \u{1F600}\u{1F680} world \u{1F468}\u{200D}\u{1F4BB}";
        db.save_entry(emoji_text).unwrap();

        let entries = db.list_entries(1000).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].content, emoji_text);
    }

    #[test]
    fn test_use_entry_not_found() {
        let db = Database::open_in_memory().unwrap();
        let err = db.use_entry(9999).unwrap_err();
        assert!(matches!(err, crate::error::FclipError::NotFound(9999)));
    }

    #[test]
    fn test_toggle_pin_not_found() {
        let db = Database::open_in_memory().unwrap();
        let err = db.toggle_pin(9999, "label".to_string()).unwrap_err();
        assert!(matches!(err, crate::error::FclipError::NotFound(9999)));
    }

    #[test]
    fn test_hash_content_deterministic() {
        assert_eq!(hash_content("abc"), hash_content("abc"));
        assert_ne!(hash_content("abc"), hash_content("def"));
    }

    #[test]
    fn test_hash_content_is_sha256() {
        let hash = hash_content("hello");
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
