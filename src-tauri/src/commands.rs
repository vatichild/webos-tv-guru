use std::path::PathBuf;
use std::sync::Arc;

use tauri::ipc::Channel;
use tauri::State;

use crate::apps::{InstallProgress, InstalledApp};
use crate::devices::{DeviceInfo, NewDevice};
use crate::error::{AppError, AppResult};
use crate::sync::{SyncConfig, SyncEntry, SyncManager};
use crate::{apps, devices};

/// SSH work blocks, so it runs off the async runtime.
async fn blocking<T: Send + 'static>(
    f: impl FnOnce() -> AppResult<T> + Send + 'static,
) -> AppResult<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| AppError::other(e.to_string()))?
}

#[tauri::command]
pub async fn list_devices() -> AppResult<Vec<DeviceInfo>> {
    blocking(devices::list).await
}

#[tauri::command]
pub async fn add_device(device: NewDevice) -> AppResult<DeviceInfo> {
    blocking(move || devices::add(device)).await
}

#[tauri::command]
pub async fn update_device(
    sync: State<'_, Arc<SyncManager>>,
    original: String,
    device: NewDevice,
) -> AppResult<DeviceInfo> {
    let old = original.clone();
    let updated = blocking(move || devices::update(&original, device)).await?;
    if updated.name != old {
        sync.device_renamed(&old, Some(&updated.name))?;
    }
    Ok(updated)
}

#[tauri::command]
pub async fn remove_device(sync: State<'_, Arc<SyncManager>>, name: String) -> AppResult<()> {
    let removed = name.clone();
    blocking(move || devices::remove(&name)).await?;
    sync.device_renamed(&removed, None)
}

#[tauri::command]
pub async fn set_default_device(name: String) -> AppResult<()> {
    blocking(move || devices::set_default(&name)).await
}

/// Connect and list apps, so a passing test means installs will work too.
#[tauri::command]
pub async fn test_device(name: String) -> AppResult<usize> {
    blocking(move || {
        let session = devices::connect(&name)?;
        Ok(apps::list_installed(&session)?.len())
    })
    .await
}

#[tauri::command]
pub async fn list_apps(device: String) -> AppResult<Vec<InstalledApp>> {
    blocking(move || apps::list_installed(&devices::connect(&device)?)).await
}

#[tauri::command]
pub async fn install_app(
    device: String,
    path: PathBuf,
    on_progress: Channel<InstallProgress>,
) -> AppResult<String> {
    blocking(move || {
        let session = devices::connect(&device)?;
        apps::install(&session, &path, |p| {
            let _ = on_progress.send(p);
        })
    })
    .await
}

#[tauri::command]
pub async fn remove_app(device: String, id: String) -> AppResult<()> {
    blocking(move || apps::remove(&devices::connect(&device)?, &id)).await
}

#[tauri::command]
pub async fn launch_app(device: String, id: String) -> AppResult<()> {
    blocking(move || apps::launch(&devices::connect(&device)?, &id)).await
}

#[tauri::command]
pub fn sync_get_config(sync: State<'_, Arc<SyncManager>>) -> SyncConfig {
    sync.config()
}

#[tauri::command]
pub fn sync_set_config(sync: State<'_, Arc<SyncManager>>, config: SyncConfig) -> AppResult<()> {
    sync.set_config(config)
}

#[tauri::command]
pub fn sync_entries(sync: State<'_, Arc<SyncManager>>) -> Vec<SyncEntry> {
    sync.entries()
}

/// Rescan now; `install` also installs missing and outdated apps.
#[tauri::command]
pub fn sync_now(sync: State<'_, Arc<SyncManager>>, install: bool) {
    sync.trigger(install);
}
