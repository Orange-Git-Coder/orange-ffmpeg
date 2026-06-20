use std::path::PathBuf;
use std::process::{Command, Stdio};

pub struct Ffmpeg {
    bin: String,
}

impl Ffmpeg {
    pub fn new(bin: &str) -> Self {
        Self { bin: bin.into() }
    }

    pub fn convert(
        &self,
        input: &PathBuf,
        output: &PathBuf,
        codec: &str,
        crf: &str,
        preset: &str,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-i").arg(input);
        c.arg("-c:v").arg(codec);
        c.arg("-crf").arg(crf);
        c.arg("-preset").arg(preset);
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn compress(&self, input: &PathBuf, output: &PathBuf, target_mb: f64) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-i").arg(input);
        // Two-pass target size approximation
        c.arg("-b:v").arg(format!("{}k", (target_mb * 8192.0 / 10.0) as u32));
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn to_gif(
        &self,
        input: &PathBuf,
        output: &PathBuf,
        start: &str,
        duration: &str,
        fps: u32,
        width: u32,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-ss").arg(start).arg("-t").arg(duration).arg("-i").arg(input);
        c.arg("-vf")
            .arg(format!("fps={},scale={}:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse", fps, width));
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn extract_audio(&self, input: &PathBuf, output: &PathBuf, codec: &str) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-i").arg(input).arg("-vn");
        c.arg("-c:a").arg(codec);
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn merge_av(
        &self,
        video: &PathBuf,
        audio: &PathBuf,
        output: &PathBuf,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-i").arg(video).arg("-i").arg(audio);
        c.arg("-c:v").arg("copy").arg("-c:a").arg("aac");
        c.arg("-shortest").arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn burn_subtitle(
        &self,
        input: &PathBuf,
        sub: &PathBuf,
        output: &PathBuf,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-i").arg(input);
        c.arg("-vf").arg(format!("subtitles={}", sub.display()));
        c.arg("-c:a").arg("copy");
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }

    pub fn screenshot(
        &self,
        input: &PathBuf,
        output: &PathBuf,
        at_time: &str,
    ) -> Command {
        let mut c = Command::new(&self.bin);
        c.arg("-y").arg("-ss").arg(at_time).arg("-i").arg(input);
        c.arg("-frames:v").arg("1").arg("-q:v").arg("2");
        c.arg(output);
        c.stderr(Stdio::piped()).stdout(Stdio::null()).stdin(Stdio::null());
        c
    }
}
