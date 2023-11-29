use std::process::Command;

use anyhow::{bail, Result};
use log::info;

pub struct YtDlp {}

impl YtDlp {
    pub fn get_binary_path() -> Result<String> {
        let output = Command::new("which")
            .arg("yt-dlp")
            .output()
            .expect("can't find `yt-dlp` binary in your system.");

        if !output.status.success() {
            bail!("can't find `yt-dlp` binary in your system.");
        }
        let yt_dlp_path = String::from_utf8(output.stdout)?;
        Ok(yt_dlp_path)
    }

    /// yt-dlp https://www.youtube.com/watch\?v\=L3MjPtK7ZP8 --write-sub --write-auto-sub --sub-lang "en" --sub-format json3
    pub fn download_subtitle(video_url: &str) -> Result<String> {
        let output = Command::new("yt-dlp")
            .arg(video_url)
            .arg("--write-sub")
            .arg("--write-auto-sub")
            .arg("--sub-lang")
            .arg("en")
            .arg("--sub-format")
            .arg("json3")
            .arg("-o")
            .arg("./static/%(id)s.%(ext)s")
            .output()
            .expect("can't find `yt-dlp` binary in your system.");

        if !output.status.success() {
            bail!("{}", String::from_utf8(output.stderr)?);
        }

        info!("out {}", String::from_utf8(output.stdout)?);
        Ok("".to_string())
    }
}
