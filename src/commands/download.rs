use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use crate::tools::ytdlp::Ytdlp;
use anyhow::Result;

pub struct DownloadCommand;

impl Command for DownloadCommand {
    fn name(&self) -> &str {
        "/download"
    }
    fn aliases(&self) -> &[&str] {
        &["/dl", "/d"]
    }
    fn description(&self) -> &str {
        "下载视频 (yt-dlp + aria2c)"
    }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        let url = args.first().cloned().unwrap_or_default();
        if url.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /download <URL>");
            return Ok(());
        }

        state.push_output(OutputKind::Command, &format!("/download {}", url));
        state.push_output(OutputKind::Info, "正在获取视频信息...");

        let ytdlp = Ytdlp::new("yt-dlp");
        match ytdlp.get_info(&url) {
            Ok(info) => {
                state.push_output(OutputKind::Success, &format!("标题: {}", info.title));
                state.push_output(
                    OutputKind::Result,
                    &format!("  时长: {}", info.duration_str),
                );
                state.push_output(OutputKind::Result, &format!("  封面: {}", info.thumbnail));
                state.push_output(OutputKind::Info, "可用格式:");
                for f in &info.formats {
                    state.push_output(
                        OutputKind::Result,
                        &format!(
                            "  [{}] {} {} {} {:?}",
                            f.id, f.resolution, f.codec, f.ext, f.filesize
                        ),
                    );
                }
                if !info.audio_only.is_empty() {
                    state.push_output(OutputKind::Info, "纯音频:");
                    for a in &info.audio_only {
                        state.push_output(
                            OutputKind::Result,
                            &format!("  [{}] {} {} {:?}", a.id, a.codec, a.ext, a.filesize),
                        );
                    }
                }
                state.push_output(OutputKind::Info, "提示: /download <URL> <格式ID> 开始下载");
            }
            Err(e) => {
                state.push_output(OutputKind::Error, &format!("获取失败: {}", e));
            }
        }
        Ok(())
    }
}
