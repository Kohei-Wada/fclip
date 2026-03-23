use crate::config::{Config, KeybindingsConfig};
use crate::db::Database;
use crate::error::FclipError;
use crate::search::{FuzzySearcher, SearchResult};
use std::sync::Arc;
use tauri::State;

pub struct AppState {
    pub db: Arc<Database>,
    pub config: Config,
    pub searcher: FuzzySearcher,
}

#[tauri::command]
pub fn search_clipboard(
    query: String,
    state: State<AppState>,
) -> Result<Vec<SearchResult>, FclipError> {
    use crate::constants::MAX_SEARCH_RESULTS;
    let limit = if query.is_empty() {
        MAX_SEARCH_RESULTS
    } else {
        state.config.behavior.max_history
    };
    let entries = state.db.list_entries(limit)?;
    Ok(state.searcher.search(&entries, &query, MAX_SEARCH_RESULTS))
}

#[tauri::command]
pub fn paste_entry(id: i64, state: State<AppState>) -> Result<(), FclipError> {
    let content = state.db.use_entry(id)?;
    clipboard_win::set_clipboard_string(&content)
        .map_err(|e| FclipError::Clipboard(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn delete_entry(id: i64, state: State<AppState>) -> Result<(), FclipError> {
    state.db.delete_entry(id)
}

#[tauri::command]
pub fn toggle_pin(id: i64, label: String, state: State<AppState>) -> Result<bool, FclipError> {
    state.db.toggle_pin(id, label)
}

#[tauri::command]
pub fn get_keybindings(state: State<AppState>) -> KeybindingsConfig {
    state.config.keybindings.clone()
}
