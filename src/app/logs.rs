use crate::AppWindow;
use slint::{ComponentHandle, SharedString};

pub fn setup_logs_callbacks(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.on_clear_logs(move || {
        tracing::info!("Clear logs requested");

        let manager = crate::core::logs::LogManager::new();

        match manager.clear() {
            Ok(()) => {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from("Logs cleared successfully."));
                    ui.set_toast_variant(1);
                    ui.set_show_toast(true);
                }
            }
            Err(err) => {
                tracing::error!("Failed to clear logs: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Failed to clear logs: {}",
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
            }
        }
    });
    let ui_weak = ui.as_weak();
    ui.on_export_logs(move || {
        tracing::info!("Export logs requested");

        let manager = crate::core::logs::LogManager::new();

        let export_path = crate::core::config::CavarluxConfig::data_dir()
            .join("logs")
            .join("cavarlux_export.log");

        if let Some(parent) = export_path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                tracing::error!("Failed to create log directory: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Failed to create log directory: {}",
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
                return;
            }
        }

        match manager.export(&export_path) {
            Ok(()) => {
                tracing::info!("Logs exported to {}", export_path.display());
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Logs exported to {}",
                        export_path.display()
                    )));
                    ui.set_toast_variant(1);
                    ui.set_show_toast(true);
                }
            }
            Err(err) => {
                tracing::error!("Log export failed: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Export failed: {}",
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
            }
        }
    });
}
