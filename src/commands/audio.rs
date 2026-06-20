use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct AudioCommand;

impl Command for AudioCommand {
    fn name(&self) -> &str { "/audio" }
    fn aliases(&self) -> &[&str] { &["/a"] }
    fn description(&self) -> &str { "提取音频 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /audio <输入> [格式]");
            state.push_output(OutputKind::Info, "支持格式: mp3, aac, flac, wav (默认 mp3)");
            state.push_output(OutputKind::Info, "示例: /audio video.mp4 mp3");
            return Ok(());
        }

        let input = &args[0];
        let fmt = args.get(1).map(|s| s.as_str()).unwrap_or("mp3");
        let output = format!(
            "{}.{}",
            std::path::Path::new(input).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default(),
            fmt
        );

        state.push_output(OutputKind::Command, &format!("/audio {} → {}", input, output));
        state.push_output(OutputKind::Info, &format!("提取音频: ffmpeg -i {} -vn -c:a {} {}", input, fmt, output));
        state.push_output(OutputKind::Success, "音频提取命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
