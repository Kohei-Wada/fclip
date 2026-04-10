/**
 * Create a dummy history.db for screenshots.
 *
 * Usage:
 *   node scripts/seed_dummy_db.mjs            # create dummy DB
 *   node scripts/seed_dummy_db.mjs --restore   # restore original DB
 */

import { createHash } from "node:crypto";
import { existsSync, mkdirSync, renameSync, unlinkSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { DatabaseSync } from "node:sqlite";

const DB_DIR = join(homedir(), "AppData", "Local", "fclip");
const DB_PATH = join(DB_DIR, "history.db");
const BAK_PATH = join(DB_DIR, "history.db.bak");

const sha256 = (text) => createHash("sha256").update(text).digest("hex");

const nowMinus = (minutes) =>
  new Date(Date.now() - minutes * 60_000).toISOString();

// [content, minutes_ago, pinned, label]
const ENTRIES = [
  ["https://github.com/nicedayzhu/fclip", 2, false, ""],
  ["npm run dev", 5, false, ""],
  ['const [query, setQuery] = useState("")', 8, false, ""],
  ["SELECT * FROM clipboard_entries ORDER BY last_used_at DESC", 12, false, ""],
  ["cargo build --release", 15, false, ""],
  ["C:\\Users\\kohei\\AppData\\Local\\fclip\\history.db", 20, false, ""],
  ["git log --oneline -20", 25, false, ""],
  ["docker compose up -d", 30, false, ""],
  ['fn main() {\n    println!("Hello, world!");\n}', 35, false, ""],
  ["192.168.1.100", 40, false, ""],
  ["The quick brown fox jumps over the lazy dog", 50, false, ""],
  ["kubectl get pods -n production", 55, false, ""],
  ["ssh-keygen -t ed25519 -C 'kohei@fclip'", 60, true, "SSH"],
  ["Bearer eyJhbGciOiJIUzI1NiIs...", 70, false, ""],
  ["https://docs.rs/rusqlite/latest/rusqlite/", 80, true, "rusqlite docs"],
  ["pip install -r requirements.txt", 90, false, ""],
  ["#[derive(Debug, Clone, Serialize)]", 100, false, ""],
  ["export RUST_LOG=debug", 110, true, "debug env"],
  ["Lorem ipsum dolor sit amet, consectetur adipiscing elit.", 120, false, ""],
  ["npx tauri build", 130, false, ""],
];

function createDummyDb() {
  mkdirSync(DB_DIR, { recursive: true });

  if (existsSync(DB_PATH)) {
    if (existsSync(BAK_PATH)) {
      console.error(`Backup already exists: ${BAK_PATH}`);
      console.error(
        "Restore first with: node scripts/seed_dummy_db.mjs --restore",
      );
      process.exit(1);
    }
    renameSync(DB_PATH, BAK_PATH);
    console.log(`Backed up: ${DB_PATH} -> ${BAK_PATH}`);
  }

  const db = new DatabaseSync(DB_PATH);
  db.exec(`
    CREATE TABLE IF NOT EXISTS clipboard_entries (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      content TEXT NOT NULL,
      content_hash TEXT UNIQUE NOT NULL,
      created_at TEXT NOT NULL,
      last_used_at TEXT NOT NULL,
      pinned INTEGER NOT NULL DEFAULT 0,
      label TEXT NOT NULL DEFAULT ''
    );
    CREATE INDEX IF NOT EXISTS idx_last_used
      ON clipboard_entries(last_used_at DESC);
  `);

  const insert = db.prepare(`
    INSERT INTO clipboard_entries
      (content, content_hash, created_at, last_used_at, pinned, label)
    VALUES (?, ?, ?, ?, ?, ?)
  `);

  for (const [content, minsAgo, pinned, label] of ENTRIES) {
    const ts = nowMinus(minsAgo);
    insert.run(content, sha256(content), ts, ts, pinned ? 1 : 0, label);
  }

  db.close();
  console.log(`Created dummy DB with ${ENTRIES.length} entries: ${DB_PATH}`);
}

function restore() {
  if (!existsSync(BAK_PATH)) {
    console.error("No backup found. Nothing to restore.");
    process.exit(1);
  }
  if (existsSync(DB_PATH)) unlinkSync(DB_PATH);
  renameSync(BAK_PATH, DB_PATH);
  console.log(`Restored: ${BAK_PATH} -> ${DB_PATH}`);
}

if (process.argv.includes("--restore")) {
  restore();
} else {
  createDummyDb();
}
