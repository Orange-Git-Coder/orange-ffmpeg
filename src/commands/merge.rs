use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct MergeCommand;

impl Command for MergeCommand {
    fn name(&self) -> &str { "/merge" }
    fn aliases(&self) -> &[&str] { &["/m"] }
    fn description(&self) -> &str { "合并视频+音频 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.len() < 2 {
            state.push_output(OutputKind::Warn, "用法: /merge <视频文件> <音频文件> [输出]");
            state.push_output(OutputKind::Info, "示例: /merge video.mp4 audio.m4a output.mp4");
            return Ok(());
        }

        let video = &args[0];
        let audio = &args[1];
        let output = if args.len() > 2 { args[2].clone() } else {
            format!("merged_{}", std::path::Path::new(video).file_name().map(|s| s.to_string_lossy()).unwrap_or_else(|| "output.mp4".into()))
        };

        state.push_output(OutputKind::Command, &format!("/merge {} + {} → {}", video, audio, output));
        state.push_output(OutputKind::Info, "合并音视频: 视频流 copy + 音频流 aac 编码");
        state.push_output(OutputKind::Success, "合并命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
