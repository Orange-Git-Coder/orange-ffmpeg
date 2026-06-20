use crate::commands::Command;
use crate::state::{AppState, OutputKind};
use crate::tools::ffprobe::Ffprobe;
use anyhow::Result;
use std::path::PathBuf;

pub struct InfoCommand;

impl Command for InfoCommand {
    fn name(&self) -> &str {
        "/info"
    }
    fn aliases(&self) -> &[&str] {
        &["/i", "/probe"]
    }
    fn description(&self) -> &str {
        "查看媒体文件信息 (ffprobe)"
    }

    fn execute(&self, state: &mut AppState, args: &[String]) -> Result<()> {
        let path_str = args.first().cloned().unwrap_or_default();
        if path_str.is_empty() {
            state.push_output(OutputKind::Warn, "用法: /info <文件路径>");
            return Ok(());
        }

        let path = PathBuf::from(&path_str);
        state.push_output(OutputKind::Command, &format!("/info {}", path_str));

        let probe = Ffprobe::new("ffprobe");
        match probe.probe(&path) {
            Ok(r) => {
                state.push_output(OutputKind::Success, &format!("文件: {}", r.filename));
                state.push_output(OutputKind::Result, &format!("  格式: {}", r.format_name));
                state.push_output(OutputKind::Result, &format!("  时长: {:.1}s", r.duration));
                state.push_output(
                    OutputKind::Result,
                    &format!("  大小: {:.1}MB", r.size as f64 / 1_048_576.0),
                );
                state.push_output(
                    OutputKind::Result,
                    &format!("  码率: {}kbps", r.bit_rate / 1000),
                );
                state.push_output(OutputKind::Info, "视频流:");
                state.push_output(
                    OutputKind::Result,
                    &format!(
                        "  编码: {}  分辨率: {}x{}  帧率: {:.2}",
                        r.video_codec, r.width, r.height, r.fps
                    ),
                );
                if !r.audio_streams.is_empty() {
                    state.push_output(OutputKind::Info, "音频流:");
                    for a in &r.audio_streams {
                        state.push_output(
                            OutputKind::Result,
                            &format!(
                                "  语言: {}  编码: {}  声道: {}",
                                a.language, a.codec, a.channels
                            ),
                        );
                    }
                }
                if !r.subtitle_streams.is_empty() {
                    state.push_output(OutputKind::Info, "字幕流:");
                    for s in &r.subtitle_streams {
                        state.push_output(OutputKind::Result, &format!("  语言: {}", s.language));
                    }
                }
            }
            Err(e) => {
                state.push_output(OutputKind::Error, &format!("探测失败: {}", e));
            }
        }
        Ok(())
    }
}
