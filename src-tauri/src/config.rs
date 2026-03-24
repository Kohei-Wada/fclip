use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub hotkey: HotkeyConfig,
    #[serde(default)]
    pub behavior: BehaviorConfig,
    #[serde(default)]
    pub keybindings: KeybindingsConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HotkeyConfig {
    #[serde(default = "default_hotkey")]
    pub open: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BehaviorConfig {
    #[serde(default = "default_max_history")]
    pub max_history: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeybindingsConfig {
    #[serde(default = "default_select")]
    pub select: String,
    #[serde(default = "default_close")]
    pub close: String,
    #[serde(default = "default_delete")]
    pub delete: String,
    #[serde(default = "default_next")]
    pub next: String,
    #[serde(default = "default_prev")]
    pub prev: String,
    #[serde(default = "default_backspace")]
    pub backspace: String,
    #[serde(default = "default_clear")]
    pub clear: String,
}

fn default_hotkey() -> String {
    "Ctrl+Shift+V".to_string()
}
fn default_max_history() -> usize {
    1000
}
fn default_select() -> String {
    "Enter".to_string()
}
fn default_close() -> String {
    "Escape".to_string()
}
fn default_delete() -> String {
    "Ctrl+d".to_string()
}
fn default_next() -> String {
    "Ctrl+n,Ctrl+j".to_string()
}
fn default_prev() -> String {
    "Ctrl+p,Ctrl+k".to_string()
}
fn default_backspace() -> String {
    "Ctrl+h".to_string()
}
fn default_clear() -> String {
    "Ctrl+u".to_string()
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            open: default_hotkey(),
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            max_history: default_max_history(),
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Key {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct KeybindingsResponse {
    pub select: Vec<Key>,
    pub close: Vec<Key>,
    pub delete: Vec<Key>,
    pub next: Vec<Key>,
    pub prev: Vec<Key>,
    pub backspace: Vec<Key>,
    pub clear: Vec<Key>,
}

fn parse_key(s: &str) -> Key {
    let parts: Vec<&str> = s.trim().split('+').collect();
    let key = parts.last().unwrap_or(&"").to_lowercase();
    if key.is_empty() {
        eprintln!("[fclip] WARNING: Empty keybinding string: '{}'", s);
    }
    let has = |m: &str| parts.iter().any(|p| p.eq_ignore_ascii_case(m));
    Key {
        key,
        ctrl: has("ctrl"),
        shift: has("shift"),
        alt: has("alt"),
        meta: has("meta"),
    }
}

fn parse_bindings(s: &str) -> Vec<Key> {
    s.split(',')
        .map(|b| b.trim())
        .filter(|b| !b.is_empty())
        .map(parse_key)
        .collect()
}

impl KeybindingsConfig {
    pub fn to_response(&self) -> KeybindingsResponse {
        KeybindingsResponse {
            select: parse_bindings(&self.select),
            close: parse_bindings(&self.close),
            delete: parse_bindings(&self.delete),
            next: parse_bindings(&self.next),
            prev: parse_bindings(&self.prev),
            backspace: parse_bindings(&self.backspace),
            clear: parse_bindings(&self.clear),
        }
    }
}

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            select: default_select(),
            close: default_close(),
            delete: default_delete(),
            next: default_next(),
            prev: default_prev(),
            backspace: default_backspace(),
            clear: default_clear(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(crate::constants::APP_DIR_NAME)
            .join(crate::constants::CONFIG_FILENAME)
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => Self::from_toml(&content),
                Err(e) => {
                    eprintln!(
                        "[fclip] WARNING: Failed to read config {}: {}",
                        path.display(),
                        e
                    );
                    Self::default()
                }
            }
        } else {
            Self::default()
        }
    }

    pub fn from_toml(content: &str) -> Self {
        toml::from_str(content).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.hotkey.open, "Ctrl+Shift+V");
        assert_eq!(config.behavior.max_history, 1000);

        assert_eq!(config.keybindings.select, "Enter");
        assert_eq!(config.keybindings.close, "Escape");
        assert_eq!(config.keybindings.delete, "Ctrl+d");
        assert_eq!(config.keybindings.next, "Ctrl+n,Ctrl+j");
        assert_eq!(config.keybindings.prev, "Ctrl+p,Ctrl+k");
        assert_eq!(config.keybindings.backspace, "Ctrl+h");
        assert_eq!(config.keybindings.clear, "Ctrl+u");
    }

    #[test]
    fn test_from_toml_empty() {
        let config = Config::from_toml("");
        assert_eq!(config.hotkey.open, "Ctrl+Shift+V");
        assert_eq!(config.behavior.max_history, 1000);
    }

    #[test]
    fn test_from_toml_partial() {
        let toml = r#"
[hotkey]
open = "Alt+V"

[behavior]
max_history = 500
"#;
        let config = Config::from_toml(toml);
        assert_eq!(config.hotkey.open, "Alt+V");
        assert_eq!(config.behavior.max_history, 500);

        assert_eq!(config.keybindings.select, "Enter");
    }

    #[test]
    fn test_from_toml_invalid_returns_default() {
        let config = Config::from_toml("this is not valid toml {{{");
        assert_eq!(config.hotkey.open, "Ctrl+Shift+V");
    }

    #[test]
    fn test_from_toml_custom_keybindings() {
        let toml = r#"
[keybindings]
select = "Ctrl+Enter"
next = "Ctrl+j"
prev = "Ctrl+k"
"#;
        let config = Config::from_toml(toml);
        assert_eq!(config.keybindings.select, "Ctrl+Enter");
        assert_eq!(config.keybindings.next, "Ctrl+j");
        assert_eq!(config.keybindings.prev, "Ctrl+k");
        assert_eq!(config.keybindings.close, "Escape");
        assert_eq!(config.keybindings.delete, "Ctrl+d");
    }

    #[test]
    fn test_parse_key_simple() {
        let key = parse_key("Enter");
        assert_eq!(
            key,
            Key {
                key: "enter".into(),
                ctrl: false,
                shift: false,
                alt: false,
                meta: false
            }
        );
    }

    #[test]
    fn test_parse_key_single_modifier() {
        let key = parse_key("Ctrl+d");
        assert_eq!(
            key,
            Key {
                key: "d".into(),
                ctrl: true,
                shift: false,
                alt: false,
                meta: false
            }
        );
    }

    #[test]
    fn test_parse_key_multiple_modifiers() {
        let key = parse_key("Ctrl+Shift+V");
        assert_eq!(
            key,
            Key {
                key: "v".into(),
                ctrl: true,
                shift: true,
                alt: false,
                meta: false
            }
        );
    }

    #[test]
    fn test_parse_key_case_insensitive() {
        let key = parse_key("ctrl+D");
        assert_eq!(
            key,
            Key {
                key: "d".into(),
                ctrl: true,
                shift: false,
                alt: false,
                meta: false
            }
        );
    }

    #[test]
    fn test_parse_key_with_whitespace() {
        let key = parse_key("  Ctrl+d  ");
        assert_eq!(
            key,
            Key {
                key: "d".into(),
                ctrl: true,
                shift: false,
                alt: false,
                meta: false
            }
        );
    }

    #[test]
    fn test_parse_bindings_single() {
        let bindings = parse_bindings("Ctrl+n");
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].key, "n");
        assert!(bindings[0].ctrl);
    }

    #[test]
    fn test_parse_bindings_multiple() {
        let bindings = parse_bindings("Ctrl+n,Ctrl+j");
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].key, "n");
        assert_eq!(bindings[1].key, "j");
        assert!(bindings[0].ctrl);
        assert!(bindings[1].ctrl);
    }

    #[test]
    fn test_parse_bindings_filters_empty_entries() {
        let bindings = parse_bindings("Ctrl+n,,Ctrl+j");
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].key, "n");
        assert_eq!(bindings[1].key, "j");
    }

    #[test]
    fn test_parse_bindings_empty_string() {
        let bindings = parse_bindings("");
        assert_eq!(bindings.len(), 0);
    }

    #[test]
    fn test_parse_bindings_whitespace_around_commas() {
        let bindings = parse_bindings("Ctrl+n , Ctrl+j");
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings[0].key, "n");
        assert_eq!(bindings[1].key, "j");
    }

    #[test]
    fn test_keybindings_response_default_values() {
        let config = Config::default();
        let resp = config.keybindings.to_response();
        assert_eq!(resp.select.len(), 1);
        assert_eq!(resp.select[0].key, "enter");
        assert_eq!(resp.next.len(), 2);
        assert_eq!(resp.next[0].key, "n");
        assert_eq!(resp.next[1].key, "j");
        assert!(resp.next[0].ctrl);
        assert!(resp.next[1].ctrl);
        assert_eq!(resp.prev.len(), 2);
        assert_eq!(resp.prev[0].key, "p");
        assert_eq!(resp.prev[1].key, "k");
    }

    #[test]
    fn test_keybindings_response_multiple_values() {
        let toml = r#"
[keybindings]
next = "Ctrl+n,Ctrl+j"
prev = "Ctrl+p,Ctrl+k"
"#;
        let config = Config::from_toml(toml);
        let resp = config.keybindings.to_response();
        assert_eq!(resp.next.len(), 2);
        assert_eq!(resp.next[0].key, "n");
        assert_eq!(resp.next[1].key, "j");
        assert_eq!(resp.prev.len(), 2);
        assert_eq!(resp.prev[0].key, "p");
        assert_eq!(resp.prev[1].key, "k");
        assert_eq!(resp.select.len(), 1);
        assert_eq!(resp.select[0].key, "enter");
    }

    #[test]
    fn test_from_toml_custom_keybindings_response() {
        let toml = r#"
[keybindings]
select = "Ctrl+Enter"
next = "Ctrl+j"
prev = "Ctrl+k"
"#;
        let config = Config::from_toml(toml);
        let resp = config.keybindings.to_response();
        assert_eq!(resp.select[0].key, "enter");
        assert!(resp.select[0].ctrl);
        assert_eq!(resp.next[0].key, "j");
        assert_eq!(resp.prev[0].key, "k");
    }
}
