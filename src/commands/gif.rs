use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct GifCommand;

impl Command for GifCommand {
    fn name(&self) -> &str { "/gif" }
    fn aliases(&self) -> &[&str] { &[] }
    fn description(&self) -> &str { "视频转 GIF" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.len() < 3 {
            state.push_output(OutputKind::Warn, "用法: /gif <输入> <开始时间> <时长> [FPS] [宽度]");
            state.push_output(OutputKind::Info, "示例: /gif input.mp4 00:10 5 15 480");
            return Ok(());
        }

        let input = &args[0];
        let start = &args[1];
        let dur = &args[2];
        let fps: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(15);
        let width: u32 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(480);

        let output = format!(
            "{}_gif.gif",
            std::path::Path::new(input).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default()
        );

        state.push_output(OutputKind::Command, &format!("/gif {} {}s → {}", input, dur, output));
        state.push_output(OutputKind::Info, &format!("参数: start={} dur={} fps={} width={}", start, dur, fps, width));
        state.push_output(OutputKind::Success, "GIF 命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
