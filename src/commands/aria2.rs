use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use anyhow::Result;

pub struct Aria2Command;

impl Command for Aria2Command {
    fn name(&self) -> &str { "/aria2" }
    fn aliases(&self) -> &[&str] { &["/dl2"] }
    fn description(&self) -> &str { "通用下载 (aria2c): HTTP/FTP/磁力/种子" }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        let url = args.first().cloned().unwrap_or_default();
        if url.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /aria2 <URL|磁力链接|种子文件>");
            state.push_output(OutputKind::Info, "支持: HTTP, HTTPS, FTP, magnet:, .torrent");
            state.push_output(OutputKind::Info, "示例: /aria2 https://example.com/file.zip");
            return Ok(());
        }

        state.push_output(OutputKind::Command, &format!("/aria2 {}", url));
        state.push_output(OutputKind::Info, "使用 aria2c 下载 (16线程, 16分片)");
        state.push_output(OutputKind::Success, "下载命令已构建 (执行待接入异步引擎)");
        Ok(())
    }
}
