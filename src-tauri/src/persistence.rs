// persistence.rs — State + Config persistence via tauri-plugin-store
// Implements US-005: state survives app restart

use hermesx_core::config::UserConfig;
use hermesx_core::state_machine::PersistedState;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "hermesx.json";
const KEY_STATE:  &str = "work_state";
const KEY_CONFIG: &str = "user_config";

pub fn save_state(app: &AppHandle, state: &PersistedState) {
    if let Ok(store) = app.store(STORE_FILE) {
        let _ = store.set(KEY_STATE, serde_json::to_value(state).unwrap_or_default());
        let _ = store.save();
    }
}

pub fn load_state(app: &AppHandle) -> PersistedState {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_STATE))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

pub fn save_config(app: &AppHandle, config: &UserConfig) {
    if let Ok(store) = app.store(STORE_FILE) {
        let _ = store.set(KEY_CONFIG, serde_json::to_value(config).unwrap_or_default());
        let _ = store.save();
    }
}

pub fn load_config(app: &AppHandle) -> UserConfig {
    app.store(STORE_FILE)
        .ok()
        .and_then(|s| s.get(KEY_CONFIG))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}
