use crate::db::{self, Database, InsertResult};
use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

fn get_clipboard_text() -> Option<String> {
    clipboard_win::get_clipboard_string().ok()
}

pub struct ClipboardWatcher {
    db: Arc<Database>,
    max_history: usize,
    last_hash: Mutex<Option<String>>,
}

impl ClipboardWatcher {
    pub fn new(db: Arc<Database>, max_history: usize) -> Self {
        Self {
            db,
            max_history,
            last_hash: Mutex::new(None),
        }
    }

    /// Check if text is new (not a duplicate of the last seen content).
    /// Returns Some(text) if new, None if duplicate or empty.
    pub fn check_new_content(&self, text: String) -> Option<String> {
        if text.is_empty() || text.len() > crate::constants::MAX_CONTENT_SIZE {
            return None;
        }

        let hash = db::hash_content(&text);
        let mut last = self.last_hash.lock().ok()?;

        if last.as_deref() == Some(&hash) {
            return None;
        }
        *last = Some(hash);
        drop(last);

        Some(text)
    }

    fn handle_new_content(watcher: &Arc<Self>, app_handle: &AppHandle, text: String) {
        match watcher.db.save_entry(&text) {
            Ok(result) => {
                let _ = app_handle.emit("clipboard-updated", ());
                if result == InsertResult::New {
                    let _ = watcher.db.enforce_history_limit(watcher.max_history);
                }
            }
            Err(e) => log::error!("Failed to save clipboard entry: {}", e),
        }
    }

    pub fn start(self: Arc<Self>, app_handle: AppHandle) {
        let watcher = Arc::clone(&self);

        std::thread::spawn(move || {
            struct Handler {
                watcher: Arc<ClipboardWatcher>,
                app_handle: AppHandle,
            }

            impl ClipboardHandler for Handler {
                fn on_clipboard_change(&mut self) -> CallbackResult {
                    let text = match get_clipboard_text() {
                        Some(t) => t,
                        None => return CallbackResult::Next,
                    };
                    if let Some(text) = self.watcher.check_new_content(text) {
                        ClipboardWatcher::handle_new_content(
                            &self.watcher,
                            &self.app_handle,
                            text,
                        );
                    }
                    CallbackResult::Next
                }
            }

            let handler = Handler {
                watcher,
                app_handle,
            };

            let mut master = Master::new(handler);
            if let Err(e) = master.run() {
                log::error!("Clipboard listener error: {}", e);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_watcher() -> ClipboardWatcher {
        let db = Arc::new(Database::open_in_memory().unwrap());
        ClipboardWatcher::new(db, 1000)
    }

    #[test]
    fn test_check_new_content_returns_new() {
        let watcher = make_watcher();
        let result = watcher.check_new_content("hello".to_string());
        assert_eq!(result, Some("hello".to_string()));
    }

    #[test]
    fn test_check_new_content_ignores_duplicate() {
        let watcher = make_watcher();
        watcher.check_new_content("hello".to_string());
        let result = watcher.check_new_content("hello".to_string());
        assert_eq!(result, None);
    }

    #[test]
    fn test_check_new_content_detects_change() {
        let watcher = make_watcher();
        watcher.check_new_content("hello".to_string());
        let result = watcher.check_new_content("world".to_string());
        assert_eq!(result, Some("world".to_string()));
    }

    #[test]
    fn test_check_new_content_empty_string() {
        let watcher = make_watcher();
        let result = watcher.check_new_content("".to_string());
        assert_eq!(result, None);
    }
}
