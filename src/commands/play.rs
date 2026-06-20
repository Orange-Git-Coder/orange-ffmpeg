use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use crate::tools::ffplay::Ffplay;
use anyhow::Result;
use std::path::PathBuf;

pub struct PlayCommand;

impl Command for PlayCommand {
    fn name(&self) -> &str { "/play" }
    fn aliases(&self) -> &[&str] { &["/p"] }
    fn description(&self) -> &str { "播放视频/音频 (ffplay)" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        let path_str = args.first().cloned().unwrap_or_default();
        if path_str.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /play <文件路径>");
            return Ok(());
        }

        let path = PathBuf::from(&path_str);
        if !path.exists() {
            state.push_output(OutputKind::Error, &format!("文件不存在: {}", path_str));
            return Ok(());
        }

        state.push_output(OutputKind::Command, &format!("/play {}", path_str));

        let ffplay = Ffplay::new("ffplay");
        match ffplay.play(&path) {
            Ok(()) => {
                state.push_output(OutputKind::Success, &format!("已启动播放: {}", path_str));
                state.push_output(OutputKind::Info, "ffplay 快捷键: 空格暂停, ←→ 快进/退, f 全屏, q 退出");
            }
            Err(e) => {
                state.push_output(OutputKind::Error, &format!("播放失败: {}", e));
            }
        }
        Ok(())
    }
}
