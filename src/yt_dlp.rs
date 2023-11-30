use std::process::Command;

use crate::json3::Json3Subtitle;
use anyhow::{bail, Result};
use log::info;
use qstring::QString;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct YtVidoInfo {
    id: String,
    title: String,
    thumbnail: String,
    description: String,
    channel_url: String,
    #[serde(rename = "channel")]
    channel_name: String,
    view_count: u64,
}

impl YtVidoInfo {
    pub fn new(json_dump: &str) -> Result<Self> {
        let vid_info: YtVidoInfo = serde_json::from_str(json_dump)?;
        Ok(vid_info)
    }
}

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

    pub fn fetch_video_id_from_url(url: &str) -> Result<String> {
        let a = url.split("watch").collect::<Vec<&str>>()[1];
        let q_str = QString::from(a);
        let a = q_str.get("v").expect("can't get the video id from url");
        Ok(a.to_string())
    }

    /// TODO: Add way to delete downloaded content after regular interval or use --dump-single-json.
    pub fn download_subtitle(video_url: &str) -> Result<String> {
        let video_id = Self::fetch_video_id_from_url(video_url)?;
        let p = format!("./static/{}.en.json", video_id);
        let cached_path = Path::new(&p);
        if !cached_path.exists() {
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
                .arg("--skip-download")
                .output()
                .expect("can't find `yt-dlp` binary in your system.");

            if !output.status.success() {
                bail!("{}", String::from_utf8(output.stderr)?);
            }
            info!("out {}", String::from_utf8(output.stdout)?);
        }

        let json3_text =
            std::fs::read_to_string(format!("./static/{}.en.json3", video_id)).unwrap();
        let json3: Json3Subtitle = serde_json::from_str(json3_text.as_str()).unwrap();
        Ok(json3.to_string())
    }

    pub fn get_video_info(video_url: &str) -> Result<YtVidoInfo> {
        let output = Command::new("yt-dlp")
            .arg(video_url)
            .arg("--dump-single-json")
            .arg("--skip-download")
            .output()
            .expect("can't find `yt-dlp` binary in your system.");

        if !output.status.success() {
            bail!("{}", String::from_utf8(output.stderr)?);
        }

        let json_dump = String::from_utf8(output.stdout)?;
        let vid_info = YtVidoInfo::new(&json_dump)?;

        info!("out {:?}", &vid_info);
        Ok(vid_info)
    }
}
