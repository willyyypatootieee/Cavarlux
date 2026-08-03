use crate::AppWindow;
use slint::{ComponentHandle, SharedString};

pub fn setup_settings_callbacks(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.on_toggle_setting(move |key: SharedString, value: bool| {
        tracing::info!("Setting changed: {} = {}", key, value);

        let mut config = crate::core::config::CavarluxConfig::load();

        match key.as_str() {
            "dark_mode" => {
                config.theme = if value {
                    "dark".to_owned()
                } else {
                    "light".to_owned()
                };
            }
            "check_updates" | "auto_update" => {
                config.check_updates = value;
            }
            "open_at_login" => {
                config.open_at_login = value;
            }
            "developer_mode" => {
                config.developer_mode = value;
            }
            "show_advanced" => {
                config.show_advanced = value;
            }
            "debug_logging" => {
                config.debug_logging = value;
            }
            "memory_optimization" => {
                config.memory_optimization = value;
            }
            "zram_optimization" => {
                config.zram_optimization = value;
            }
            "kill_idle_wine" => {
                config.kill_idle_wine = value;
            }
            "auto_memory_trim" => {
                config.auto_memory_trim = value;
            }
            "governor_boost" => {
                config.governor_boost = value;
            }
            unknown_key => {
                tracing::warn!("Unknown setting key: {}", unknown_key);
            }
        }

        match config.save() {
            Ok(()) => {
                tracing::info!("Config saved successfully");
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Setting '{}' saved to config.toml",
                        key
                    )));
                    ui.set_toast_variant(1);
                    ui.set_show_toast(true);
                }
            }
            Err(err) => {
                tracing::error!("Failed to save config: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Failed to save setting '{}': {}",
                        key,
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
            }
        }
    });


    let ui_weak = ui.as_weak();
    ui.on_toggle_wine(move |key: SharedString, value: bool| {
        tracing::info!("Wine setting changed: {} = {}", key, value);

        let mut config = crate::core::config::CavarluxConfig::load();

        match key.as_str() {
            "dxvk" => config.dxvk_enabled = value,
            "esync" => config.esync_enabled = value,
            "fsync" => config.fsync_enabled = value,
            "gpu-accel" => config.gpu_accel_enabled = value,
            "wine-debug" => config.wine_debug_enabled = value,
            unknown => {
                tracing::warn!("Unknown wine setting: {}", unknown);
                return;
            }
        }

        if let Err(err) = config.save() {
            tracing::error!("Failed to save wine config: {}", err);
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_toast_message(SharedString::from(std::format!(
                    "Failed to save setting '{}': {}",
                    key,
                    err
                )));
                ui.set_toast_variant(3);
                ui.set_show_toast(true);
            }
            return;
        }

        // Reflect in UI immediately.
        if let Some(ui) = ui_weak.upgrade() {
            match key.as_str() {
                "dxvk" => ui.set_wine_dxvk_enabled(value),
                "esync" => ui.set_wine_esync_enabled(value),
                "fsync" => ui.set_wine_fsync_enabled(value),
                "gpu-accel" => ui.set_wine_gpu_accel_enabled(value),
                "wine-debug" => ui.set_wine_debug_enabled(value),
                _ => {}
            }
            ui.set_toast_message(SharedString::from("Wine setting saved."));
            ui.set_toast_variant(1);
            ui.set_show_toast(true);
        }
    });

    // ──────────────────────────────────────────────
    // Stop Wine (manual kill of our prefix wineserver)
    // ──────────────────────────────────────────────
    let ui_weak = ui.as_weak();
    ui.on_kill_wine(move || {
        tracing::info!("Stop Wine requested");
        match crate::system::wine_process::WineProcessManager::kill_all() {
            Ok(message) => {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(message));
                    ui.set_toast_variant(1);
                    ui.set_show_toast(true);
                }
            }
            Err(err) => {
                tracing::error!("Stop Wine failed: {}", err);
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_toast_message(SharedString::from(std::format!(
                        "Stop Wine failed: {}",
                        err
                    )));
                    ui.set_toast_variant(3);
                    ui.set_show_toast(true);
                }
            }
        }
    });


    let ui_weak = ui.as_weak();
    ui.on_select_accent(move |accent_name: SharedString| {
        tracing::info!("Accent palette changed to: {}", accent_name);
        let mut config = crate::core::config::CavarluxConfig::load();
        config.accent_color = accent_name.to_string();
        if let Err(err) = config.save() {
            tracing::error!("Failed to save accent color config: {}", err);
            return;
        }

        if let Some(ui) = ui_weak.upgrade() {
            ui.set_setting_active_accent(accent_name.clone());
            ui.set_toast_message(SharedString::from(format!(
                "Accent color updated to {}",
                accent_name
            )));
            ui.set_toast_variant(1);
            ui.set_show_toast(true);
        }
    });
}
