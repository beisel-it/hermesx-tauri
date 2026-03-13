//! OS keychain integration via the `keyring` crate.
//!
//! Service name: "HermesX"
//! Entry:        username → stored as keyring entry, password in keyring secret
//!
//! On Windows: Credential Manager
//! On macOS:   Keychain
//! On Linux:   libsecret / kwallet via keyring crate

use keyring::Entry;

const SERVICE: &str = "HermesX";
const USERNAME_KEY: &str = "zeusX.username";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoredCredentials {
    pub username: String,
    pub password: String,
}

pub fn save_credentials(username: &str, password: &str) -> Result<(), String> {
    // Store username as a well-known key's secret (lightweight)
    let user_entry = Entry::new(SERVICE, USERNAME_KEY).map_err(|e| e.to_string())?;
    user_entry.set_password(username).map_err(|e| e.to_string())?;

    // Store password under the actual username as entry
    let pass_entry = Entry::new(SERVICE, username).map_err(|e| e.to_string())?;
    pass_entry.set_password(password).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_credentials() -> Option<StoredCredentials> {
    let user_entry = Entry::new(SERVICE, USERNAME_KEY).ok()?;
    let username   = user_entry.get_password().ok()?;

    let pass_entry = Entry::new(SERVICE, &username).ok()?;
    let password   = pass_entry.get_password().ok()?;

    Some(StoredCredentials { username, password })
}

pub fn delete_credentials() -> Result<(), String> {
    if let Ok(entry) = Entry::new(SERVICE, USERNAME_KEY) {
        if let Ok(username) = entry.get_password() {
            if let Ok(pass_entry) = Entry::new(SERVICE, &username) {
                let _ = pass_entry.delete_credential();
            }
        }
        let _ = entry.delete_credential();
    }
    Ok(())
}
