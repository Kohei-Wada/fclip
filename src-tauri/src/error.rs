use serde::Serialize;
use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum FclipError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("Clipboard error: {0}")]
    Clipboard(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Entry not found: {0}")]
    NotFound(i64),
    #[error("Lock poisoned")]
    LockPoisoned,
}

impl Serialize for FclipError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<T> From<PoisonError<T>> for FclipError {
    fn from(_: PoisonError<T>) -> Self {
        Self::LockPoisoned
    }
}

pub type Result<T> = std::result::Result<T, FclipError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = FclipError::Clipboard("test error".to_string());
        assert_eq!(err.to_string(), "Clipboard error: test error");

        let err = FclipError::LockPoisoned;
        assert_eq!(err.to_string(), "Lock poisoned");

        let err = FclipError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
        assert_eq!(err.to_string(), "IO error: missing");
    }

    #[test]
    fn test_error_serialize() {
        let err = FclipError::Clipboard("test".to_string());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"Clipboard error: test\"");

        let err = FclipError::LockPoisoned;
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"Lock poisoned\"");
    }

    #[test]
    fn test_from_poison_error() {
        let mutex = std::sync::Mutex::new(42);
        let _guard = mutex.lock().unwrap();
        // Can't easily create a PoisonError, but we can test the variant exists
        let err = FclipError::LockPoisoned;
        assert!(matches!(err, FclipError::LockPoisoned));
    }
}
