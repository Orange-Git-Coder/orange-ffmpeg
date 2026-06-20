# 🍊 orange-cli

> AI-powered FFmpeg terminal toolbox — Claude Code 风格美学

一个基于 Rust 的终端多媒体工具箱，集成 **FFmpeg / FFplay / FFprobe / yt-dlp / aria2c**，提供统一的命令面板式交互界面。

## ✨ 特性

- **命令面板** — 输入 `/` 调出模糊搜索面板，↑↓ 选择，Tab 补全
- **历史记录** — ↑↓ 浏览命令历史
- **统一交互** — 所有工具共用一个终端界面，无需切换
- **美观 UI** — 参考 Claude Code 设计语言，暖橙色调，深色主题

## 📦 内置命令

| 命令 | 别名 | 描述 |
|------|------|------|
| `/download` | `/dl`, `/d` | 下载视频 (yt-dlp + aria2c) |
| `/info` | `/i`, `/probe` | 查看媒体文件信息 (ffprobe) |
| `/play` | — | 播放视频/音频 (ffplay) |
| `/convert` | — | 视频格式转换 (ffmpeg) |
| `/compress` | — | 压缩视频 (ffmpeg) |
| `/gif` | — | 视频转 GIF |
| `/audio` | — | 提取音频 (ffmpeg) |
| `/merge` | — | 合并视频+音频 (ffmpeg) |
| `/subtitle` | — | 烧录字幕到视频 (ffmpeg) |
| `/screenshot` | — | 视频截图 (ffmpeg) |
| `/aria2` | — | 通用下载 (aria2c): HTTP/FTP/磁力/种子 |

## 🔧 前置依赖

确保以下工具已安装并在 PATH 中可用：

- [FFmpeg](https://ffmpeg.org/) — `ffmpeg`, `ffplay`, `ffprobe`
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) — 视频下载
- [aria2](https://aria2.github.io/) — `aria2c` 多线程下载

## 🚀 安装

```bash
# 克隆仓库
git clone https://github.com/Orange-Git-Coder/orange-ffmpeg.git
cd orange-ffmpeg

# 编译运行（需要 Rust 1.80+）
cargo build --release
./target/release/orange-cli
```

## 🎮 快捷键

| 按键 | 功能 |
|------|------|
| `/` | 打开命令面板 |
| `↑↓` | 浏览历史 / 选择命令 |
| `Tab` | 自动补全 |
| `Esc` | 清空输入 |
| `Enter` | 执行命令 |
| `q` (输入为空时) | 退出 |

## 🖼️ 界面布局

```
┌─ 🍊 orange-cli v0.1.0 ─── 11 项命令 ─┐
├────────────────────────────────────────┤
│  ℹ 输入 / 浏览命令                     │
│  ✓ 下载完成: video.mp4                 │
│  ...                                   │
├────────────────────────────────────────┤
│  命令 (5)                              │
│  ▶ /download  下载视频 (yt-dlp)        │
│    /info      查看媒体文件信息          │
│  ...                                   │
├────────────────────────────────────────┤
│  ▎ /download █                         │
│  / 命令面板  ↑↓ 历史  Tab 补全         │
└────────────────────────────────────────┘
```

## 🛠️ 技术栈

- **Rust** — 安全、高性能
- **ratatui** — 终端 UI 框架
- **crossterm** — 跨平台终端控制
- **tokio** — 异步运行时

## 📄 License

MIT
