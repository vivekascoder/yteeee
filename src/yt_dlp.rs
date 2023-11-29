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

    pub fn download_subtitle(video_url: &str) -> Result<String> {
        let output = Command::new("yt-dlp")
            .arg(video_url)
            .arg("--write-subs")
            .output()
            .expect("can't find `yt-dlp` binary in your system.");
        // info!("running {}"÷);

        if !output.status.success() {
            bail!("{}", String::from_utf8(output.stderr)?);
        }
        Ok("".to_string())
    }
}
