use crate::core::config::CavarluxConfig;
use std::process::Command;

pub struct EnvRepair;

impl EnvRepair {
    pub fn repair_env_vars() -> Result<String, String> {
        let prefix = CavarluxConfig::prefix_path();

        let output = Command::new("wine")
            .env("WINEPREFIX", &prefix)
            .args(&["cmd", "/c", "set", "DXVK_HUD=0"])
            .output()
            .map_err(|e| format!("Failed to set wine env vars: {}", e))?;

        if output.status.success() {
            Ok("Wine environment variables successfully reset.".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
