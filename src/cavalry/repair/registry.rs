use crate::core::config::CavarluxConfig;
use std::process::Command;

pub struct RegistryRepair;

impl RegistryRepair {
    pub fn reset_registry_keys() -> Result<String, String> {
        let prefix = CavarluxConfig::prefix_path();

        let output = Command::new("wine")
            .env("WINEPREFIX", &prefix)
            .args(&["reg", "add", "HKCU\\Software\\Mainframe\\Cavalry", "/f"])
            .output()
            .map_err(|e| format!("Failed to reset registry keys: {}", e))?;

        if output.status.success() {
            Ok("Wine registry keys successfully restored to defaults.".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
