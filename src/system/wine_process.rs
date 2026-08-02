use crate::core::config::CavarluxConfig;
use std::process::Command;

pub struct WineProcessManager;

impl WineProcessManager {
    /// True if a wineserver is currently running for our prefix.
    pub fn wineserver_running() -> bool {
        let prefix = CavarluxConfig::prefix_path();
        Command::new("wineserver")
            .env("WINEPREFIX", &prefix)
            .arg("-p")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn active_apps() -> Vec<String> {
        let prefix = CavarluxConfig::prefix_path();
        let prefix_str = prefix.to_string_lossy();
        let patterns = [
            "Cavalry.exe",
            "winecfg",
            "regedit",
            "msiexec",
            "explorer.exe",
            "winedbg",
            "taskmgr",
        ];
        let mut apps = Vec::new();

        for pattern in patterns {
            let Ok(output) = Command::new("pgrep").args(["-f", pattern]).output() else {
                continue;
            };
            if !output.status.success() {
                continue;
            }
            let ids = String::from_utf8_lossy(&output.stdout);
            for id in ids.split_whitespace() {
                if let Ok(pid) = id.parse::<u32>() {
                    if pid != std::process::id() {
                        let env_path = format!("/proc/{}/environ", pid);
                        let is_cavarlux_process = match std::fs::read_to_string(&env_path) {
                            Ok(env_str) => {
                                env_str.split('\0').any(|var| {
                                    if let Some((k, v)) = var.split_once('=') {
                                        k == "WINEPREFIX" && v == prefix_str
                                    } else {
                                        false
                                    }
                                })
                            }
                            Err(_) => false,
                        };

                        if is_cavarlux_process {
                            apps.push(pattern.to_string());
                            break;
                        }
                    }
                }
            }
        }

        apps
    }

    pub fn is_idle() -> bool {
        Self::wineserver_running() && Self::active_apps().is_empty()
    }

    pub fn has_active_apps() -> bool {
        !Self::active_apps().is_empty()
    }

    pub fn kill_all() -> Result<String, String> {
        let prefix = CavarluxConfig::prefix_path();
        tracing::info!(
            "Stopping idle Wine processes for Cavarlux prefix {:?}...",
            prefix
        );

        let output = Command::new("wineserver")
            .env("WINEPREFIX", &prefix)
            .arg("-k")
            .output()
            .map_err(|e| format!("Failed to stop Wine: {}", e))?;

        if !output.status.success() {
            return Err("wineserver -k did not exit cleanly.".to_string());
        }

        Ok("Idle Cavarlux Wine processes stopped and memory reclaimed.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_active_apps_no_self_match() {
        let apps = WineProcessManager::active_apps();
        assert!(!apps.iter().any(|a| a == "Cavalry.exe"));
    }

    #[test]
    fn test_kill_when_not_running() {
        if !WineProcessManager::wineserver_running() {
            assert!(WineProcessManager::kill_all().is_ok());
        }
    }
}
