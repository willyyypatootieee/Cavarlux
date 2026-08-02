use std::env;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub distro: String,
    pub distro_id: String,
    pub distro_family: String,
    pub desktop: String,
    pub display: String,
    pub gpu: String,
    pub wine_version: String,
    pub cavalry_installed: bool,
    pub cavalry_version: String,
    pub prefix_healthy: bool,
    pub prefix_path: String,
}

impl SystemInfo {
    pub fn fetch() -> Self {
        let wine_ver = detect_wine_version();
        let prefix = crate::core::config::CavarluxConfig::prefix_path();
        let prefix_exists = prefix.exists();

        let cavalry_exe = prefix.join("drive_c/Program Files/Cavalry/Cavalry.exe");
        let cavalry_installed = cavalry_exe.exists();

        let distro_info = crate::system::distro::DistroInfo::detect();

        Self {
            distro: detect_distro(),
            distro_id: distro_info.id.clone(),
            distro_family: format!("{:?}", distro_info.family),
            desktop: detect_desktop(),
            display: detect_display(),
            gpu: detect_gpu(),
            wine_version: wine_ver,
            cavalry_installed,
            cavalry_version: if cavalry_installed {
                "1.5.8".to_string()
            } else {
                "Not Installed".to_string()
            },
            prefix_healthy: prefix_exists,
            prefix_path: prefix.to_string_lossy().to_string(),
        }
    }
}

pub fn detect_distro() -> String {
    for path in &["/etc/os-release", "/usr/lib/os-release"] {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let name = line
                        .trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string();
                    if !name.is_empty() {
                        return name;
                    }
                }
            }
        }
    }
    "Linux".to_string()
}

pub fn detect_desktop() -> String {
    for key in &[
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
        "XDG_SESSION_DESKTOP",
    ] {
        if let Ok(val) = env::var(key) {
            if !val.trim().is_empty() {
                return val.trim().to_string();
            }
        }
    }
    "Custom Desktop".to_string()
}

pub fn detect_display() -> String {
    if let Ok(val) = env::var("XDG_SESSION_TYPE") {
        let lower = val.trim().to_lowercase();
        if lower == "wayland" {
            return "Wayland".to_string();
        } else if lower == "x11" {
            return "X11".to_string();
        }
    }
    if env::var("WAYLAND_DISPLAY").is_ok() {
        return "Wayland".to_string();
    }
    if env::var("DISPLAY").is_ok() {
        return "X11".to_string();
    }
    "Linux Display".to_string()
}

pub fn detect_gpu() -> String {
    if let Ok(output) = Command::new("lspci").output() {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut gpus = Vec::new();
            for line in stdout.lines() {
                let lower = line.to_lowercase();
                if lower.contains("vga compatible controller")
                    || lower.contains("3d controller")
                    || lower.contains("display controller")
                {
                    if let Some(idx) = line.find(": ") {
                        let raw = line[idx + 2..].trim();
                        gpus.push(clean_gpu_name(raw));
                    }
                }
            }
            if !gpus.is_empty() {
                if let Some(discrete) = gpus
                    .iter()
                    .find(|g| g.contains("NVIDIA") || g.contains("Radeon") || g.contains("GeForce"))
                {
                    return discrete.clone();
                }
                return gpus[0].clone();
            }
        }
    }

    "Graphics Device".to_string()
}

fn clean_gpu_name(raw: &str) -> String {
    if let (Some(start), Some(end)) = (raw.find('['), raw.find(']')) {
        let model = &raw[start + 1..end];
        if raw.contains("NVIDIA") {
            return format!("NVIDIA {}", model);
        } else if raw.contains("AMD") || raw.contains("Advanced Micro Devices") {
            return format!("AMD {}", model);
        } else if raw.contains("Intel") {
            return format!("Intel {}", model);
        }
        return model.to_string();
    }
    raw.to_string()
}

pub fn detect_wine_version() -> String {
    if let Ok(output) = Command::new("wine").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !version.is_empty() {
                return version.replace("wine-", "Wine ");
            }
        }
    }
    "Not Installed".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_fetch() {
        let info = SystemInfo::fetch();
        assert!(!info.distro.is_empty());
        assert!(!info.desktop.is_empty());
        assert!(!info.display.is_empty());
        assert!(!info.gpu.is_empty());
        assert!(!info.wine_version.is_empty());
    }
}
