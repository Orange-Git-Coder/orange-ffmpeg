use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Ffplay {
    bin: String,
}

impl Ffplay {
    pub fn new(bin: &str) -> Self {
        Self { bin: bin.into() }
    }

    /// Spawn ffplay in a separate window (detached process)
    pub fn play(&self, path: &PathBuf) -> Result<()> {
        let child = Command::new(&self.bin)
            .arg("-window_title")
            .arg(format!("orange-cli: {}", path.display()))
            .arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()
            .context("ffplay 启动失败")?;

        // Don't wait — let ffplay run independently
        drop(child);
        Ok(())
    }

    pub fn play_with_args(&self, path: &PathBuf, args: &[&str]) -> Result<()> {
        let mut cmd = Command::new(&self.bin);
        for a in args {
            cmd.arg(a);
        }
        cmd.arg(path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()
            .context("ffplay 启动失败")?;

        Ok(())
    }
}
