use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Aria2 {
    bin: String,
    max_conn: u32,
    split: u32,
}

impl Aria2 {
    pub fn new(bin: &str, max_conn: u32, split: u32) -> Self {
        Self { bin: bin.into(), max_conn, split }
    }

    pub fn download(
        &self,
        url: &str,
        dir: &PathBuf,
        name: &str,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("--max-connection-per-server").arg(self.max_conn.to_string());
        c.arg("--split").arg(self.split.to_string());
        c.arg("--dir").arg(dir);
        c.arg("--out").arg(name);
        c.arg("--enable-rpc=false");
        c.arg("--console-log-level=notice");
        c.arg("--show-console-readout=true");
        c.arg("--summary-interval=1");
        c.arg("--seed-time=0");
        c.arg(url);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn parse_progress(line: &str) -> Option<Aria2Progress> {
        if !line.contains("ETA:") { return None; }
        let pct = line.split('%').next()?
            .split(|c: char| !c.is_ascii_digit()).last()?
            .parse::<u8>().ok()?;
        let speed = line.split("speed:").nth(1)?
            .trim().split(' ').next().unwrap_or("?").to_string();
        let eta = line.split("ETA:").nth(1)?.trim().to_string();
        Some(Aria2Progress { progress: pct.min(100), speed, eta })
    }
}

#[derive(Debug, Clone)]
pub struct Aria2Progress {
    pub progress: u8,
    pub speed: String,
    pub eta: String,
}
