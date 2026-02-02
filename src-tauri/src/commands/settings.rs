use crate::services::settings::app_settings;
use crate::types::{AppSettings, SettingsError};
use tauri::AppHandle;

/// Get current app settings
#[tauri::command]
pub async fn get_app_settings(app_handle: AppHandle) -> Result<AppSettings, String> {
    log::info!("Loading app settings");

    app_settings::load_settings(&app_handle)
        .await
        .map_err(|e| {
            log::error!("Failed to load settings: {e}");
            match e {
                SettingsError::Corrupted => {
                    "Settings file is corrupted. Default settings will be used.".to_string()
                },
                SettingsError::FileRead(_) => {
                    "Unable to read settings file. Please check file permissions.".to_string()
                },
                SettingsError::JsonParse(_) => {
                    "Settings file format is invalid. Default settings will be used.".to_string()
                },
                _ => format!("Unable to load settings: {e}"),
            }
        })
        .map(|settings| {
            log::info!("Successfully loaded settings: {settings:?}");
            settings
        })
}

/// Update app settings
#[tauri::command]
pub async fn update_app_settings(settings: AppSettings) -> Result<(), String> {
    log::info!("Received settings update request: {settings:?}");

    // Validate and sanitize settings first
    let validated_settings =
        app_settings::validate_and_sanitize_settings(settings).map_err(|e| {
            log::error!("Settings validation failed: {e}");
            format!("Invalid settings provided: {e}")
        })?;

    log::info!("Settings validation passed, proceeding to save");

    // Save the validated settings
    app_settings::save_settings(validated_settings)
        .await
        .map_err(|e| {
            log::error!("Failed to save settings: {e}");
            format!("Unable to save settings. Please check file permissions and try again: {e}")
        })?;

    log::info!("Settings saved successfully");
    Ok(())
}

/// Reset app settings to defaults
#[tauri::command]
pub async fn reset_app_settings(app_handle: AppHandle) -> Result<AppSettings, String> {
    log::info!("Resetting app settings to defaults");

    app_settings::reset_to_defaults(&app_handle)
        .await
        .map_err(|e| {
            log::error!("Failed to reset settings: {e}");
            format!("Unable to reset settings to defaults. Please try again: {e}")
        })
        .map(|settings| {
            log::info!("Successfully reset settings to defaults: {settings:?}");
            settings
        })
}
