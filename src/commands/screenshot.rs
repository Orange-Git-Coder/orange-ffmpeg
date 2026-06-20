use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct ScreenshotCommand;

impl Command for ScreenshotCommand {
    fn name(&self) -> &str { "/screenshot" }
    fn aliases(&self) -> &[&str] { &["/ss"] }
    fn description(&self) -> &str { "视频截图 (ffmpeg)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        if args.len() < 2 {
            state.push_output(OutputKind::Warn, "用法: /screenshot <视频> <时间> [数量] [间隔]");
            state.push_output(OutputKind::Info, "单张: /screenshot video.mp4 00:10");
            state.push_output(OutputKind::Info, "批量: /screenshot video.mp4 00:10 5 2");
            return Ok(());
        }

        let input = &args[0];
        let time = &args[1];
        let count: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);

        state.push_output(OutputKind::Command, &format!("/screenshot {} at {}", input, time));

        if count == 1 {
            let output = format!(
                "{}_thumb.png",
                std::path::Path::new(input).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default()
            );
            state.push_output(OutputKind::Success, &format!("截图: {}", output));
        } else {
            let interval = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
            state.push_output(OutputKind::Info, &format!("批量截图: {} 张, 间隔 {}s", count, interval));
            for i in 0..count {
                let t = format_time_shift(time, i * interval);
                let output = format!(
                    "{}_shot_{:03}.png",
                    std::path::Path::new(input).file_stem().map(|s| s.to_string_lossy()).unwrap_or_default(),
                    i + 1
                );
                state.push_output(OutputKind::Result, &format!("  [{}/{}] {} → {}", i + 1, count, t, output));
            }
        }
        state.push_output(OutputKind::Success, "截图命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}

fn format_time_shift(base: &str, offset_secs: u32) -> String {
    let parts: Vec<u32> = base.split(':').filter_map(|s| s.parse().ok()).collect();
    let total = if parts.len() == 3 {
        parts[0] * 3600 + parts[1] * 60 + parts[2]
    } else if parts.len() == 2 {
        parts[0] * 60 + parts[1]
    } else {
        parts.first().copied().unwrap_or(0)
    };
    let t = total + offset_secs;
    format!("{:02}:{:02}:{:02}", t / 3600, (t % 3600) / 60, t % 60)
}
