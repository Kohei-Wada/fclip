/// Maximum clipboard content size to store (bytes)
pub const MAX_CONTENT_SIZE: usize = 100_000;

/// Maximum number of search results returned to frontend
pub const MAX_SEARCH_RESULTS: usize = 50;

/// Maximum number of characters to display in a single result
pub const MAX_DISPLAY_LEN: usize = 100;

/// Number of context characters shown around matches when windowing
pub const DISPLAY_CONTEXT_CHARS: usize = 20;

/// Application directory name
pub const APP_DIR_NAME: &str = "fclip";

/// Database filename
pub const DB_FILENAME: &str = "history.db";

/// Config filename
pub const CONFIG_FILENAME: &str = "config.toml";
