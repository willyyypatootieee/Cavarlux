use crate::AppWindow;
use slint::{ComponentHandle, SharedString};

pub fn setup_repair_callbacks(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.on_repair_action(move |action: SharedString| {
        tracing::info!("Repair action triggered: {}", action);

        let Some(task) = crate::cavalry::repair::RepairTask::from_str(action.as_str()) else {
            tracing::warn!("Unknown repair action: {}", action);
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_toast_message(SharedString::from(std::format!(
                    "Unknown repair action: {}",
                    action
                )));
                ui.set_toast_variant(2);
                ui.set_show_toast(true);
            }
            return;
        };

        match crate::cavalry::repair::execute_repair(task) {
            Ok(message) => {
                tracing::info!("Repair succeeded: {}", message);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(message));
                    ui.set_toast_variant(1);
                    ui.set_show_toast(true);
                }
            }
            Err(err) => {
                tracing::error!("Repair failed: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Repair failed: {}",
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
            }
        }
    });
}
