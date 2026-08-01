mod app;
mod audio;
mod config;
mod de_adaptive;
mod diagnostics;
mod ffmpeg;
mod gpu_tuner;
mod i18n;
mod installer;
mod logging;
mod logs;
mod memory;
mod path_translator;
mod perf_tuner;
mod projects;
mod qol_tools;
mod repair;
mod swap_manager;
mod sys_info;
mod tray;
mod wine;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init();

    let _config = config::GallopConfig::load();

    let ui = AppWindow::new()?;

    app::setup_callbacks(&ui);

    app::set_initial_data(&ui);

    ui.run()?;

    Ok(())
}
