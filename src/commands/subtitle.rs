use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct SubtitleCommand;

impl Command for SubtitleCommand {
    fn name(&self) -> &str { "/subtitle" }
    fn aliases(&self) -> &[&str] { &["/sub"] }
    fn description(&self) -> &str { "烧录字幕到视频 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.len() < 2 {
            state.push_output(OutputKind::Warn, "用法: /subtitle <视频> <字幕文件> [输出]");
            state.push_output(OutputKind::Info, "支持: .srt, .ass");
            state.push_output(OutputKind::Info, "示例: /subtitle video.mp4 subs.srt");
            return Ok(());
        }

        let video = &args[0];
        let sub = &args[1];
        let output = if args.len() > 2 { args[2].clone() } else {
            format!("{}_subbed.{}",
                std::path::Path::new(video).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default(),
                std::path::Path::new(video).extension().map(|s| s.to_string_lossy()).unwrap_or_else(|| "mp4".into()))
        };

        state.push_output(OutputKind::Command, &format!("/subtitle {} + {} → {}", video, sub, output));
        state.push_output(OutputKind::Info, "烧录字幕: ffmpeg subtitles 滤镜");
        state.push_output(OutputKind::Success, "字幕烧录命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
