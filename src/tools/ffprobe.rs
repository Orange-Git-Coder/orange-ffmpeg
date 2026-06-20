use anyhow::{Context, Result};
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;

pub struct Ffprobe {
    bin: String,
}

impl Ffprobe {
    pub fn new(bin: &str) -> Self {
        Self { bin: bin.into() }
    }

    pub fn probe(&self, path: &PathBuf) -> Result<ProbeResult> {
        let output = Command::new(&self.bin)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
            ])
            .arg(path)
            .output()
            .context("ffprobe 执行失败")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("ffprobe 错误: {}", stderr);
        }

        let json: Value = serde_json::from_slice(&output.stdout)
            .context("ffprobe JSON 解析失败")?;

        let mut result = ProbeResult::default();

        if let Some(fmt) = json["format"].as_object() {
            result.filename = fmt["filename"].as_str().unwrap_or("?").into();
            result.format_name = fmt["format_name"].as_str().unwrap_or("?").into();
            result.duration = fmt["duration"]
                .as_str()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            result.size = fmt["size"]
                .as_str()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            result.bit_rate = fmt["bit_rate"]
                .as_str()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
        }

        if let Some(streams) = json["streams"].as_array() {
            for st in streams {
                let codec_type = st["codec_type"].as_str().unwrap_or("?");
                match codec_type {
                    "video" => {
                        result.video_codec = st["codec_name"].as_str().unwrap_or("?").into();
                        result.width = st["width"].as_u64().unwrap_or(0);
                        result.height = st["height"].as_u64().unwrap_or(0);
                        if let Some(fps_str) = st["r_frame_rate"].as_str() {
                            result.fps = parse_fps(fps_str);
                        }
                    }
                    "audio" => {
                        let lang = st["tags"]["language"].as_str().unwrap_or("und");
                        let codec = st["codec_name"].as_str().unwrap_or("?");
                        let channels = st["channels"].as_u64().unwrap_or(0);
                        result.audio_streams.push(AudioStreamInfo {
                            language: lang.into(),
                            codec: codec.into(),
                            channels,
                        });
                    }
                    "subtitle" => {
                        let lang = st["tags"]["language"].as_str().unwrap_or("und");
                        result.subtitle_streams.push(SubtitleStreamInfo {
                            language: lang.into(),
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(result)
    }
}

fn parse_fps(s: &str) -> f64 {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() == 2 {
        let num: f64 = parts[0].parse().unwrap_or(0.0);
        let den: f64 = parts[1].parse().unwrap_or(1.0);
        if den > 0.0 { num / den } else { 0.0 }
    } else {
        s.parse().unwrap_or(0.0)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProbeResult {
    pub filename: String,
    pub format_name: String,
    pub duration: f64,
    pub size: u64,
    pub bit_rate: u64,
    pub video_codec: String,
    pub width: u64,
    pub height: u64,
    pub fps: f64,
    pub audio_streams: Vec<AudioStreamInfo>,
    pub subtitle_streams: Vec<SubtitleStreamInfo>,
}

#[derive(Debug, Clone)]
pub struct AudioStreamInfo {
    pub language: String,
    pub codec: String,
    pub channels: u64,
}

#[derive(Debug, Clone)]
pub struct SubtitleStreamInfo {
    pub language: String,
}
