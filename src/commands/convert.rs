use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct ConvertCommand;

impl Command for ConvertCommand {
    fn name(&self) -> &str { "/convert" }
    fn aliases(&self) -> &[&str] { &["/cv"] }
    fn description(&self) -> &str { "视频格式转换 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /convert <输入> <输出>");
            state.push_output(OutputKind::Info, "支持格式: mp4, mkv, avi, mov, webm");
            state.push_output(OutputKind::Info, "示例: /convert input.mkv output.mp4");
            return Ok(());
        }

        let input = &args[0];
        let output = if args.len() > 1 { args[1].clone() } else {
            let stem = std::path::Path::new(input)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "output".into());
            format!("{}.mp4", stem)
        };

        state.push_output(OutputKind::Command, &format!("/convert {} → {}", input, output));
        state.push_output(OutputKind::Info, &format!("执行: ffmpeg -i {} {}", input, output));
        state.push_output(OutputKind::Success, "转码命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
