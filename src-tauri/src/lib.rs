mod apps;
mod commands;
mod devices;
mod error;
mod ipk;
mod sync;
mod version;

use tauri::Manager;

use crate::sync::SyncManager;

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
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let sync = SyncManager::new(app.handle().clone());
            sync.trigger(false);
            app.manage(sync);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_devices,
            commands::add_device,
            commands::update_device,
            commands::remove_device,
            commands::set_default_device,
            commands::test_device,
            commands::list_apps,
            commands::install_app,
            commands::remove_app,
            commands::launch_app,
            commands::sync_get_config,
            commands::sync_set_config,
            commands::sync_entries,
            commands::sync_now,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
