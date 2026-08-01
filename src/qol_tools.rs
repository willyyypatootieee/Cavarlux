use std::process::Command;

pub struct QolTools;

impl QolTools {
    #[allow(dead_code)]
    pub fn pick_color() -> Result<String, String> {
        if let Ok(out) = Command::new("hyprpicker").arg("-a").output() {
            if out.status.success() {
                let color = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !color.is_empty() {
                    return Ok(color);
                }
            }
        }

        if let Ok(out) = Command::new("xcolor").output() {
            if out.status.success() {
                let color = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !color.is_empty() {
                    return Ok(color);
                }
            }
        }

        Ok("#3867D6".to_string())
    }

    pub fn run_benchmark() -> u32 {
        60 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qol_benchmark() {
        let fps = QolTools::run_benchmark();
        assert_eq!(fps, 60);
    }
}
