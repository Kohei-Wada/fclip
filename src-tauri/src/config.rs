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
    "Ctrl+n".to_string()
}
fn default_prev() -> String {
    "Ctrl+p".to_string()
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

impl Default for KeybindingsConfig {
    fn default() -> Self {
        Self {
            select: default_select(),
            close: default_close(),
            delete: default_delete(),
            next: default_next(),
            prev: default_prev(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("fclip")
            .join("config.toml")
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
        assert_eq!(config.keybindings.next, "Ctrl+n");
        assert_eq!(config.keybindings.prev, "Ctrl+p");
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
}
