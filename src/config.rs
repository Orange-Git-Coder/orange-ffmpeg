use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE: &str = "orange-cli.toml";

/// Resolve a tool binary path relative to the executable's assets/ directory.
/// Falls back to the plain name (PATH lookup) if the asset file doesn't exist.
fn asset_path(name: &str) -> String {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));
    if let Some(dir) = exe_dir {
        let candidate = dir.join("assets").join(name);
        if candidate.exists() {
            return candidate.to_string_lossy().to_string();
        }
    }
    // Fallback: try project-root assets/ (for cargo run)
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let candidate = PathBuf::from(&manifest).join("assets").join(name);
        if candidate.exists() {
            return candidate.to_string_lossy().to_string();
        }
    }
    name.to_string()
}

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
            ffmpeg_path: asset_path("ffmpeg.exe"),
            ffprobe_path: asset_path("ffprobe.exe"),
            ffplay_path: asset_path("ffplay.exe"),
            ytdlp_path: asset_path("yt-dlp.exe"),
            aria2_path: asset_path("aria2c.exe"),
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
