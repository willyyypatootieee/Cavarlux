use crate::AppWindow;
use slint::{ComponentHandle, SharedString};

pub fn setup_gpu_callbacks(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.on_select_gpu(move |label: SharedString| {
        let label = label.to_string();
        tracing::info!("GPU selection requested: {}", label);

        let selection = crate::system::gpu_tuner::GpuTuner::key_for_label(&label);

        if selection == "auto" {
            if let Some(ui) = ui_weak.upgrade() {
                save_selection(&ui, &selection);
            }
            return;
        }

        // Manual selection → system-wide switch requiring logout.
        let target = crate::system::gpu_tuner::GpuTuner::resolve_selection(
            &selection,
            crate::system::gpu_tuner::GpuTuner::detect_gpus(),
        );
        let label = match target {
            Some(device) => device.model.clone(),
            None => selection.clone(),
        };

        crate::app::pending::set(crate::app::pending::PendingAction::GpuSwitch(selection));
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_confirm_title(SharedString::from("Switch system GPU?"));
            ui.set_confirm_message(SharedString::from(std::format!(
                "Switch to '{}'?\n\nThis changes your system graphics adapter \
                 and requires logging out and back in to take effect. \
                 Sudo/root may be requested while switching.",
                label
            )));
            ui.set_confirm_destructive(false);
            ui.set_show_confirm_dialog(true);
        }
    });
}

pub fn run_confirmed_switch(ui: &AppWindow, target: &str) {
    let device = crate::system::gpu_tuner::GpuTuner::resolve_selection(
        target,
        crate::system::gpu_tuner::GpuTuner::detect_gpus(),
    );

    let Some(device) = device else {
        ui.set_toast_message(SharedString::from(std::format!(
            "Could not resolve GPU '{}'.",
            target
        )));
        ui.set_toast_variant(3);
        ui.set_show_toast(true);
        return;
    };

    match crate::system::gpu_tuner::GpuTuner::run_switch(device) {
        Ok(message) => {
            tracing::info!("GPU switch succeeded: {}", message);
            save_selection(ui, target);
            ui.set_toast_message(SharedString::from(message));
            ui.set_toast_variant(1);
            ui.set_show_toast(true);
        }
        Err(err) => {
            tracing::error!("GPU switch failed: {}", err);
            ui.set_toast_message(SharedString::from(std::format!(
                "GPU switch failed: {}",
                err
            )));
            ui.set_toast_variant(3);
            ui.set_show_toast(true);
        }
    }
}

fn save_selection(ui: &AppWindow, selection: &str) {
    let mut config = crate::core::config::CavarluxConfig::load();
    config.gpu_selection = selection.to_string();
    if let Err(err) = config.save() {
        tracing::error!("Failed to save GPU selection: {}", err);
        ui.set_toast_message(SharedString::from(std::format!(
            "Failed to save GPU selection: {}",
            err
        )));
        ui.set_toast_variant(3);
        ui.set_show_toast(true);
        return;
    }
    ui.set_wine_gpu_selection(SharedString::from(selection));
    ui.set_wine_gpu_selection_index(gpu_selection_index(selection));
    ui.set_active_gpu(SharedString::from(
        crate::system::gpu_tuner::GpuTuner::selection_label(selection),
    ));
    ui.set_toast_message(SharedString::from(std::format!(
        "GPU selection saved: {}",
        crate::system::gpu_tuner::GpuTuner::selection_label(selection)
    )));
    ui.set_toast_variant(1);
    ui.set_show_toast(true);
}

fn gpu_selection_index(selection: &str) -> i32 {
    if selection == "auto" {
        return 0;
    }
    for (i, device) in crate::system::gpu_tuner::GpuTuner::detect_gpus()
        .iter()
        .enumerate()
    {
        if device.pci_id == selection || device.model.eq_ignore_ascii_case(selection) {
            return (i + 1) as i32;
        }
    }
    0
}
