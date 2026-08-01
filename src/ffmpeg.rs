use std::path::Path;
use std::process::Command;

#[allow(dead_code)]
pub enum ExportPreset {
    H264Mp4,
    ProRes422,
    WebmAlpha,
    OptimizedGif,
}

#[allow(dead_code)]
pub struct FfmpegProcessor;

impl FfmpegProcessor {
    #[allow(dead_code)]
    pub fn is_ffmpeg_installed() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn convert_render<P: AsRef<Path>>(
        input: P,
        output: P,
        preset: ExportPreset,
    ) -> Result<(), String> {
        let in_str = input.as_ref().to_string_lossy();
        let out_str = output.as_ref().to_string_lossy();

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-y").args(&["-i", &in_str]);

        match preset {
            ExportPreset::H264Mp4 => {
                cmd.args(&[
                    "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18", &out_str,
                ]);
            }
            ExportPreset::ProRes422 => {
                cmd.args(&["-c:v", "prores_ks", "-profile:v", "3", &out_str]);
            }
            ExportPreset::WebmAlpha => {
                cmd.args(&["-c:v", "libvpx-vp9", "-pix_fmt", "yuva420p", &out_str]);
            }
            ExportPreset::OptimizedGif => {
                cmd.args(&["-vf", "fps=30,scale=800:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse", &out_str]);
            }
        }

        let res = cmd.output().map_err(|e| e.to_string())?;
        if res.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&res.stderr).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_check() {
        let _ = FfmpegProcessor::is_ffmpeg_installed();
    }
}
