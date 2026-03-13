// credentials.rs — Credential model (storage-agnostic)
// Migrated from: src/CredentialManager.ts
//
// Storage backend is intentionally NOT here:
//   - In Tauri app: tauri-plugin-keyring (OS keychain)
//   - In tests: in-memory HashMap
// This crate only defines the shape + a testable in-memory impl.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ZeusX login credentials.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZeusXCredentials {
    pub username: String,
    pub password: String,
}

/// Validation errors for credentials.
#[derive(Debug, PartialEq)]
pub enum CredentialError {
    MissingUsername,
    MissingPassword,
    StorageError(String),
}

impl std::fmt::Display for CredentialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialError::MissingUsername => write!(f, "Username is required"),
            CredentialError::MissingPassword => write!(f, "Password is required"),
            CredentialError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

/// Trait for credential storage backends.
/// Allows swapping OS keychain ↔ in-memory for tests.
pub trait CredentialStore: Send + Sync {
    fn save(&mut self, creds: &ZeusXCredentials) -> Result<(), CredentialError>;
    fn load(&self) -> Result<Option<ZeusXCredentials>, CredentialError>;
    fn delete(&mut self) -> Result<(), CredentialError>;
}

/// In-memory credential store (tests + dry-run mode).
#[derive(Default)]
pub struct InMemoryStore {
    inner: HashMap<String, String>,
}

impl CredentialStore for InMemoryStore {
    fn save(&mut self, creds: &ZeusXCredentials) -> Result<(), CredentialError> {
        self.inner.insert("username".into(), creds.username.clone());
        self.inner.insert("password".into(), creds.password.clone());
        Ok(())
    }

    fn load(&self) -> Result<Option<ZeusXCredentials>, CredentialError> {
        match (self.inner.get("username"), self.inner.get("password")) {
            (Some(u), Some(p)) => Ok(Some(ZeusXCredentials {
                username: u.clone(),
                password: p.clone(),
            })),
            _ => Ok(None),
        }
    }

    fn delete(&mut self) -> Result<(), CredentialError> {
        self.inner.clear();
        Ok(())
    }
}

/// Validate credentials before saving.
pub fn validate(creds: &ZeusXCredentials) -> Result<(), CredentialError> {
    if creds.username.trim().is_empty() {
        return Err(CredentialError::MissingUsername);
    }
    if creds.password.is_empty() {
        return Err(CredentialError::MissingPassword);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load_roundtrip() {
        let mut store = InMemoryStore::default();
        let creds = ZeusXCredentials {
            username: "florian".into(),
            password: "hunter2".into(),
        };
        store.save(&creds).unwrap();
        let loaded = store.load().unwrap().unwrap();
        assert_eq!(loaded, creds);
    }

    #[test]
    fn load_empty_returns_none() {
        let store = InMemoryStore::default();
        assert!(store.load().unwrap().is_none());
    }

    #[test]
    fn delete_clears_store() {
        let mut store = InMemoryStore::default();
        store
            .save(&ZeusXCredentials {
                username: "x".into(),
                password: "y".into(),
            })
            .unwrap();
        store.delete().unwrap();
        assert!(store.load().unwrap().is_none());
    }

    #[test]
    fn validation_rejects_empty_username() {
        let creds = ZeusXCredentials {
            username: "  ".into(),
            password: "pw".into(),
        };
        assert_eq!(validate(&creds), Err(CredentialError::MissingUsername));
    }

    #[test]
    fn validation_rejects_empty_password() {
        let creds = ZeusXCredentials {
            username: "user".into(),
            password: "".into(),
        };
        assert_eq!(validate(&creds), Err(CredentialError::MissingPassword));
    }

    #[test]
    fn validation_passes_valid_creds() {
        let creds = ZeusXCredentials {
            username: "florian".into(),
            password: "secure".into(),
        };
        assert!(validate(&creds).is_ok());
    }
}
