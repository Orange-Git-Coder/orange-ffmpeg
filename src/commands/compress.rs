use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct CompressCommand;

impl Command for CompressCommand {
    fn name(&self) -> &str { "/compress" }
    fn aliases(&self) -> &[&str] { &["/cp"] }
    fn description(&self) -> &str { "压缩视频 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.len() < 2 {
            state.push_output(OutputKind::Warn, "用法: /compress <输入> <目标大小MB>");
            state.push_output(OutputKind::Info, "示例: /compress input.mp4 50");
            return Ok(());
        }

        let input = &args[0];
        let target_mb: f64 = args[1].parse().unwrap_or(50.0);
        let output = format!(
            "{}_compressed.{}",
            std::path::Path::new(input).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default(),
            std::path::Path::new(input).extension().map(|s| s.to_string_lossy()).unwrap_or_else(|| "mp4".into())
        );

        state.push_output(OutputKind::Command, &format!("/compress {} → {}MB", input, target_mb));
        state.push_output(OutputKind::Info, &format!("目标输出: {}", output));
        state.push_output(OutputKind::Success, "压缩命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
