use anyhow::{Context, Result};
use serde_json::Value;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct VideoInfo {
    pub title: String,
    pub duration: f64,
    pub duration_str: String,
    pub thumbnail: String,
    pub formats: Vec<FormatInfo>,
    pub audio_only: Vec<FormatInfo>,
}

#[derive(Debug, Clone)]
pub struct FormatInfo {
    pub id: String,
    pub resolution: String,
    pub codec: String,
    pub ext: String,
    pub filesize: Option<String>,
}

pub struct Ytdlp {
    bin: String,
}

impl Ytdlp {
    pub fn new(bin: &str) -> Self {
        Self { bin: bin.into() }
    }

    pub fn get_info(&self, url: &str) -> Result<VideoInfo> {
        let output = Command::new(&self.bin)
            .args(["--dump-json", "--no-playlist", url])
            .output()
            .context("yt-dlp 执行失败")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("yt-dlp 错误: {}", stderr);
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .context("yt-dlp JSON 解析失败")?;

        let title = json["title"].as_str().unwrap_or("?").to_string();
        let duration = json["duration"].as_f64().unwrap_or(0.0);
        let duration_str = format_dur(duration);
        let thumbnail = json["thumbnail"].as_str().unwrap_or("").to_string();

        let mut formats = Vec::new();
        let mut audio_only = Vec::new();

        if let Some(fmts) = json["formats"].as_array() {
            for f in fmts {
                let id = f["format_id"].as_str().unwrap_or("?").into();
                let res = f["resolution"].as_str().unwrap_or("audio only").into();
                let codec = f["vcodec"]
                    .as_str()
                    .unwrap_or(f["acodec"].as_str().unwrap_or("?"));
                let ext = f["ext"].as_str().unwrap_or("?").into();
                let size = f["filesize"].as_f64().map(|s| format_size(s));
                let is_video = f["vcodec"].as_str().map(|v| v != "none").unwrap_or(false);

                let info = FormatInfo { id, resolution: res, codec: codec.into(), ext, filesize: size };
                if is_video {
                    formats.push(info);
                } else {
                    audio_only.push(info);
                }
            }
        }

        Ok(VideoInfo { title, duration, duration_str, thumbnail, formats, audio_only })
    }
}

fn format_dur(s: f64) -> String {
    let h = (s / 3600.0) as u64;
    let m = ((s % 3600.0) / 60.0) as u64;
    let sec = (s % 60.0) as u64;
    if h > 0 { format!("{}:{:02}:{:02}", h, m, sec) } else { format!("{}:{:02}", m, sec) }
}

fn format_size(bytes: f64) -> String {
    if bytes > 1_073_741_824.0 { format!("{:.1}GB", bytes / 1_073_741_824.0) }
    else if bytes > 1_048_576.0 { format!("{:.1}MB", bytes / 1_048_576.0) }
    else if bytes > 1024.0 { format!("{:.1}KB", bytes / 1024.0) }
    else { format!("{:.0}B", bytes) }
}
