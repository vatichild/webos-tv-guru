mod apps;
mod devices;
mod error;
mod ipk;
mod sync;
mod version;

use std::path::PathBuf;
use std::sync::Arc;

use tauri::ipc::Channel;
use tauri::{Manager, State};

use crate::apps::{InstallProgress, InstalledApp};
use crate::devices::{DeviceInfo, NewDevice};
use crate::error::{AppError, AppResult};
use crate::ipk::IpkInfo;
use crate::sync::{SyncConfig, SyncEntry, SyncManager};

/// SSH work blocks, so it runs off the async runtime.
async fn blocking<T: Send + 'static>(
    f: impl FnOnce() -> AppResult<T> + Send + 'static,
) -> AppResult<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| AppError::other(e.to_string()))?
}

#[tauri::command]
async fn list_devices() -> AppResult<Vec<DeviceInfo>> {
    blocking(devices::list).await
}

#[tauri::command]
async fn add_device(device: NewDevice) -> AppResult<DeviceInfo> {
    blocking(move || devices::add(device)).await
}

#[tauri::command]
async fn update_device(
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
async fn remove_device(sync: State<'_, Arc<SyncManager>>, name: String) -> AppResult<()> {
    let removed = name.clone();
    blocking(move || devices::remove(&name)).await?;
    sync.device_renamed(&removed, None)
}

#[tauri::command]
async fn set_default_device(name: String) -> AppResult<()> {
    blocking(move || devices::set_default(&name)).await
}

/// Connect and list apps, so a passing test means installs will work too.
#[tauri::command]
async fn test_device(name: String) -> AppResult<usize> {
    blocking(move || {
        let session = devices::connect(&name)?;
        Ok(apps::list_installed(&session)?.len())
    })
    .await
}

#[tauri::command]
async fn list_apps(device: String) -> AppResult<Vec<InstalledApp>> {
    blocking(move || apps::list_installed(&devices::connect(&device)?)).await
}

#[tauri::command]
async fn install_app(
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
async fn remove_app(device: String, id: String) -> AppResult<()> {
    blocking(move || apps::remove(&devices::connect(&device)?, &id)).await
}

#[tauri::command]
async fn launch_app(device: String, id: String) -> AppResult<()> {
    blocking(move || apps::launch(&devices::connect(&device)?, &id)).await
}

#[tauri::command]
async fn read_ipk(path: PathBuf) -> AppResult<IpkInfo> {
    blocking(move || ipk::read_info(&path)).await
}

#[tauri::command]
fn sync_get_config(sync: State<'_, Arc<SyncManager>>) -> SyncConfig {
    sync.config()
}

#[tauri::command]
fn sync_set_config(sync: State<'_, Arc<SyncManager>>, config: SyncConfig) -> AppResult<()> {
    sync.set_config(config)
}

#[tauri::command]
fn sync_entries(sync: State<'_, Arc<SyncManager>>) -> Vec<SyncEntry> {
    sync.entries()
}

/// Rescan now; `install` also installs missing and outdated apps.
#[tauri::command]
fn sync_now(sync: State<'_, Arc<SyncManager>>, install: bool) {
    sync.trigger(install);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }));
    }
    builder
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let sync = SyncManager::new(app.handle().clone());
            sync.trigger(false);
            app.manage(sync);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_devices,
            add_device,
            update_device,
            remove_device,
            set_default_device,
            test_device,
            list_apps,
            install_app,
            remove_app,
            launch_app,
            read_ipk,
            sync_get_config,
            sync_set_config,
            sync_entries,
            sync_now,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
