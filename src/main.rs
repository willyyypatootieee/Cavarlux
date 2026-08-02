mod app;
mod cavalry;
mod core;
mod diagnostics;
mod system;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    core::logging::init();

    let _config = core::config::CavarluxConfig::load();
    let _ = core::projects::ProjectManager::register_linux_mime_association();

    let ui = AppWindow::new()?;

    app::setup_callbacks(&ui);

    app::set_initial_data(&ui);

    ui.run()?;

    Ok(())
}
