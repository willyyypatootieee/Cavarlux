use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Pacman,
    Zypper,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroFamily {
    Debian,
    Fedora,
    Arch,
    Suse,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DistroInfo {
    pub name: String,
    pub id: String,
    pub id_like: String,
    pub version_id: String,
    pub family: DistroFamily,
    pub package_manager: PackageManager,
}

impl DistroInfo {
    pub fn detect() -> Self {
        let mut name = String::new();
        let mut id = String::new();
        let mut id_like = String::new();
        let mut version_id = String::new();

        for path in &["/etc/os-release", "/usr/lib/os-release"] {
            if let Ok(contents) = fs::read_to_string(path) {
                for line in contents.lines() {
                    if let Some(val) = line.strip_prefix("PRETTY_NAME=") {
                        name = val.trim_matches('"').trim_matches('\'').to_string();
                    } else if let Some(val) = line.strip_prefix("ID=") {
                        id = val.trim_matches('"').trim_matches('\'').to_lowercase();
                    } else if let Some(val) = line.strip_prefix("ID_LIKE=") {
                        id_like = val.trim_matches('"').trim_matches('\'').to_lowercase();
                    } else if let Some(val) = line.strip_prefix("VERSION_ID=") {
                        version_id = val.trim_matches('"').trim_matches('\'').to_string();
                    }
                }
                if !id.is_empty() {
                    break;
                }
            }
        }

        if name.is_empty() {
            name = "Linux".to_string();
        }

        let (family, package_manager) = Self::classify(&id, &id_like);

        Self {
            name,
            id,
            id_like,
            version_id,
            family,
            package_manager,
        }
    }

    fn classify(id: &str, id_like: &str) -> (DistroFamily, PackageManager) {
        let combined = format!("{} {}", id, id_like);

        // Arch-based
        if id == "arch"
            || id == "manjaro"
            || id == "endeavouros"
            || id == "garuda"
            || id == "artix"
            || id == "cachyos"
            || combined.contains("arch")
        {
            return (DistroFamily::Arch, PackageManager::Pacman);
        }

        if id == "fedora"
            || id == "rhel"
            || id == "centos"
            || id == "rocky"
            || id == "alma"
            || id == "nobara"
            || combined.contains("fedora")
            || combined.contains("rhel")
        {
            return (DistroFamily::Fedora, PackageManager::Dnf);
        }

        if id == "debian"
            || id == "ubuntu"
            || id == "linuxmint"
            || id == "pop"
            || id == "elementary"
            || id == "zorin"
            || id == "kali"
            || id == "mx"
            || combined.contains("debian")
            || combined.contains("ubuntu")
        {
            return (DistroFamily::Debian, PackageManager::Apt);
        }

        if id == "opensuse-tumbleweed"
            || id == "opensuse-leap"
            || id == "sles"
            || combined.contains("suse")
        {
            return (DistroFamily::Suse, PackageManager::Zypper);
        }

        if which_exists("pacman") {
            return (DistroFamily::Arch, PackageManager::Pacman);
        }
        if which_exists("dnf") {
            return (DistroFamily::Fedora, PackageManager::Dnf);
        }
        if which_exists("apt") {
            return (DistroFamily::Debian, PackageManager::Apt);
        }
        if which_exists("zypper") {
            return (DistroFamily::Suse, PackageManager::Zypper);
        }

        (DistroFamily::Unknown, PackageManager::Unknown)
    }

    pub fn install_hint(&self, package: &str) -> String {
        let pkg = self.package_name(package);
        match self.package_manager {
            PackageManager::Apt => format!("sudo apt install {}", pkg),
            PackageManager::Dnf => format!("sudo dnf install {}", pkg),
            PackageManager::Pacman => format!("sudo pacman -S {}", pkg),
            PackageManager::Zypper => format!("sudo zypper install {}", pkg),
            PackageManager::Unknown => {
                format!("Install '{}' using your package manager", pkg)
            }
        }
    }

    pub fn package_name(&self, canonical: &str) -> String {
        match (canonical, self.family) {
            // Vulkan — AMD / Mesa
            ("vulkan-radeon", DistroFamily::Arch) => "vulkan-radeon".to_string(),
            ("vulkan-radeon", DistroFamily::Fedora) => "mesa-vulkan-drivers".to_string(),
            ("vulkan-radeon", DistroFamily::Debian) => "mesa-vulkan-drivers".to_string(),
            ("vulkan-radeon", DistroFamily::Suse) => "libvulkan_radeon".to_string(),

            // Vulkan — Intel
            ("vulkan-intel", DistroFamily::Arch) => "vulkan-intel".to_string(),
            ("vulkan-intel", DistroFamily::Fedora) => "mesa-vulkan-drivers".to_string(),
            ("vulkan-intel", DistroFamily::Debian) => "mesa-vulkan-drivers".to_string(),
            ("vulkan-intel", DistroFamily::Suse) => "libvulkan_intel".to_string(),

            // Vulkan tools
            ("vulkan-tools", DistroFamily::Arch) => "vulkan-tools".to_string(),
            ("vulkan-tools", DistroFamily::Debian) => "vulkan-tools".to_string(),
            ("vulkan-tools", _) => "vulkan-tools".to_string(),

            // Wine
            ("wine", DistroFamily::Arch) => "wine".to_string(),
            ("wine", DistroFamily::Fedora) => "wine".to_string(),
            ("wine", DistroFamily::Debian) => "wine".to_string(),
            ("wine", DistroFamily::Suse) => "wine".to_string(),

            _ => canonical.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn is_package_installed(&self, package: &str) -> bool {
        let pkg = self.package_name(package);
        match self.package_manager {
            PackageManager::Apt => Command::new("dpkg")
                .args(["-s", &pkg])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
            PackageManager::Dnf | PackageManager::Zypper => Command::new("rpm")
                .args(["-q", &pkg])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
            PackageManager::Pacman => Command::new("pacman")
                .args(["-Qi", &pkg])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false),
            PackageManager::Unknown => false,
        }
    }

    pub fn kernel_version() -> String {
        Command::new("uname")
            .arg("-r")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default()
    }

    pub fn kernel_version_tuple() -> (u32, u32) {
        let ver = Self::kernel_version();
        let parts: Vec<u32> = ver
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|s| s.parse().ok())
            .collect();
        (
            parts.first().copied().unwrap_or(0),
            parts.get(1).copied().unwrap_or(0),
        )
    }
}

fn which_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distro_detection() {
        let info = DistroInfo::detect();
        assert!(!info.name.is_empty());
        assert!(!info.id.is_empty());
        assert_ne!(info.family, DistroFamily::Unknown);
        assert_ne!(info.package_manager, PackageManager::Unknown);
    }

    #[test]
    fn test_install_hint_arch() {
        let arch = DistroInfo {
            name: "Arch Linux".to_string(),
            id: "arch".to_string(),
            id_like: String::new(),
            version_id: String::new(),
            family: DistroFamily::Arch,
            package_manager: PackageManager::Pacman,
        };
        assert_eq!(arch.install_hint("winetricks"), "sudo pacman -S winetricks");
        assert_eq!(arch.install_hint("wine"), "sudo pacman -S wine");
    }

    #[test]
    fn test_install_hint_debian() {
        let deb = DistroInfo {
            name: "Ubuntu 24.04".to_string(),
            id: "ubuntu".to_string(),
            id_like: "debian".to_string(),
            version_id: "24.04".to_string(),
            family: DistroFamily::Debian,
            package_manager: PackageManager::Apt,
        };
        assert_eq!(
            deb.install_hint("winetricks"),
            "sudo apt install winetricks"
        );
        assert_eq!(
            deb.install_hint("vulkan-radeon"),
            "sudo apt install mesa-vulkan-drivers"
        );
    }

    #[test]
    fn test_install_hint_fedora() {
        let fed = DistroInfo {
            name: "Fedora 40".to_string(),
            id: "fedora".to_string(),
            id_like: String::new(),
            version_id: "40".to_string(),
            family: DistroFamily::Fedora,
            package_manager: PackageManager::Dnf,
        };
        assert_eq!(fed.install_hint("wine"), "sudo dnf install wine");
    }

    #[test]
    fn test_classify_id_like() {
        let (family, pm) = DistroInfo::classify("cachyos", "arch");
        assert_eq!(family, DistroFamily::Arch);
        assert_eq!(pm, PackageManager::Pacman);

        let (family, pm) = DistroInfo::classify("linuxmint", "ubuntu debian");
        assert_eq!(family, DistroFamily::Debian);
        assert_eq!(pm, PackageManager::Apt);
    }

    #[test]
    fn test_kernel_version() {
        let ver = DistroInfo::kernel_version();
        assert!(!ver.is_empty());
        let (major, _minor) = DistroInfo::kernel_version_tuple();
        assert!(major >= 4); 
    }
}
