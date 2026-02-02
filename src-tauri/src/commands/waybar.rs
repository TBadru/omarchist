use crate::services::waybar::{
    SaveWaybarConfigPayload, WaybarConfigService, WaybarConfigSnapshot,
    WaybarProfileChangeResponse, WaybarProfileListResponse,
};
use tauri::AppHandle;

/// Load the current Waybar configuration snapshot (layout, modules, globals, passthrough).
#[tauri::command]
pub fn get_waybar_config_snapshot(app_handle: AppHandle) -> Result<WaybarConfigSnapshot, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service.load_snapshot().map_err(|err| err.to_string())
}

/// Persist Waybar configuration changes and return the refreshed snapshot.
#[tauri::command]
pub fn save_waybar_config_snapshot(
    app_handle: AppHandle,
    payload: SaveWaybarConfigPayload,
) -> Result<WaybarConfigSnapshot, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service
        .save_snapshot(&payload)
        .map_err(|err| err.to_string())
}

/// Retrieve the list of saved Waybar configuration profiles.
#[tauri::command]
pub fn list_waybar_profiles(app_handle: AppHandle) -> Result<WaybarProfileListResponse, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service.list_profiles().map_err(|err| err.to_string())
}

/// Create a new Waybar configuration profile based on the default Omarchy template.
#[tauri::command]
pub fn create_waybar_profile(
    app_handle: AppHandle,
    name: String,
) -> Result<WaybarProfileChangeResponse, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service.create_profile(&name).map_err(|err| err.to_string())
}

/// Activate an existing Waybar configuration profile and copy it into Waybar's live location.
#[tauri::command]
pub fn select_waybar_profile(
    app_handle: AppHandle,
    profile_id: String,
) -> Result<WaybarProfileChangeResponse, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service
        .select_profile(&profile_id)
        .map_err(|err| err.to_string())
}

/// Delete a stored Waybar configuration profile.
#[tauri::command]
pub fn delete_waybar_profile(
    app_handle: AppHandle,
    profile_id: String,
) -> Result<WaybarProfileChangeResponse, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service
        .delete_profile(&profile_id)
        .map_err(|err| err.to_string())
}

/// Get the current Waybar style CSS content.
#[tauri::command]
pub fn get_waybar_style_css(app_handle: AppHandle) -> Result<String, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    let snapshot = service.load_snapshot().map_err(|err| err.to_string())?;
    Ok(snapshot.style_css)
}

/// Save Waybar style CSS content to the active profile.
#[tauri::command]
pub fn save_waybar_style_css(app_handle: AppHandle, style_css: String) -> Result<String, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    let snapshot = service.load_snapshot().map_err(|err| err.to_string())?;

    // Create a payload with the current config but updated CSS
    let payload = SaveWaybarConfigPayload {
        layout: snapshot.layout,
        globals: snapshot.globals,
        modules: snapshot.modules,
        passthrough: snapshot.passthrough,
        style_css,
        module_styles: Default::default(), // Keep existing module styles
    };

    let updated_snapshot = service
        .save_snapshot(&payload)
        .map_err(|err| err.to_string())?;
    Ok(updated_snapshot.style_css)
}

/// Reset the active Waybar profile (and live files) to the bundled defaults.
#[tauri::command]
pub fn reset_waybar_to_defaults(app_handle: AppHandle) -> Result<WaybarConfigSnapshot, String> {
    let mut service = WaybarConfigService::new(&app_handle).map_err(|err| err.to_string())?;
    service
        .reset_active_profile_to_defaults()
        .map_err(|err| err.to_string())
}
