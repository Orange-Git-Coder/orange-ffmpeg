use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "orange-cli.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub download_dir: PathBuf,
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
    pub ffplay_path: String,
    pub ytdlp_path: String,
    pub aria2_path: String,
    pub aria2_max_connections: u32,
    pub aria2_split: u32,
    pub default_crf: String,
    pub default_preset: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            download_dir: PathBuf::from("./downloads"),
            ffmpeg_path: "ffmpeg".into(),
            ffprobe_path: "ffprobe".into(),
            ffplay_path: "ffplay".into(),
            ytdlp_path: "yt-dlp".into(),
            aria2_path: "aria2c".into(),
            aria2_max_connections: 16,
            aria2_split: 16,
            default_crf: "23".into(),
            default_preset: "medium".into(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .context("找不到配置目录")?
            .join("orange-cli");
        std::fs::create_dir_all(&dir).ok();
        Ok(dir.join(CONFIG_FILE))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let s = std::fs::read_to_string(&path)?;
            Ok(toml::from_str(&s)?)
        } else {
            let cfg = Self::default();
            cfg.save()?;
            Ok(cfg)
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let s = toml::to_string_pretty(self)?;
        std::fs::write(&path, s)?;
        Ok(())
    }
}
