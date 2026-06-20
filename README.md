# 🍊 orange-cli

> 终端多媒体工具箱 — 统一 FFmpeg / yt-dlp / aria2 的命令面板

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**orange-cli** 是一个基于 Rust 的 TUI（终端用户界面）多媒体工具箱。它将 FFmpeg、FFplay、FFprobe、yt-dlp、aria2c 等命令行工具整合到一个统一的交互界面中，通过命令面板（输入 `/` 唤起）快速访问所有功能。

---

## ✨ 特性

- **⌨ 命令面板** — 输入 `/` 唤起模糊搜索，`↑↓` 选择，`Tab` 一键补全
- **📜 历史记录** — `↑↓` 翻阅命令历史，支持 200 条
- **🔍 模糊搜索** — 输入部分命令名即可匹配，支持别名
- **🎨 干净 UI** — 终端原生背景，暖橙色调，`●` 圆点语义色区分输出类型
- **📦 开箱即用** — `assets/` 目录预置所有依赖工具的 Windows 二进制
- **⚡ 轻量快速** — Rust 编译，Release 仅 ~1MB

---

## 📦 内置命令（11 个）

### 视频处理

| 命令 | 别名 | 说明 |
|------|------|------|
| `/convert` | `/cv` | 视频格式转换，支持 mp4 / mkv / avi / mov / webm |
| `/compress` | `/cp` | 压缩视频文件大小 |
| `/gif` | — | 视频片段转 GIF 动图 |
| `/merge` | `/m` | 合并视频流 + 音频流 |
| `/screenshot` | — | 视频截图（指定时间点） |
| `/subtitle` | — | 烧录字幕到视频 |
| `/audio` | `/a` | 提取音频轨道 |

### 媒体信息 & 播放

| 命令 | 别名 | 说明 |
|------|------|------|
| `/info` | `/i`, `/probe` | 查看媒体文件详细信息（分辨率、码率、编码等） |
| `/play` | — | 调用 FFplay 播放视频 / 音频 |

### 下载

| 命令 | 别名 | 说明 |
|------|------|------|
| `/download` | `/dl`, `/d` | 下载在线视频（yt-dlp 解析 + aria2c 多线程加速） |
| `/aria2` | `/dl2` | 通用下载：HTTP / FTP / 磁力链接 / BT 种子 |

---

## 🎮 快捷键

| 按键 | 场景 | 功能 |
|------|------|------|
| `/` | 任意 | 打开命令面板（模糊搜索） |
| `↑` / `↓` | 普通模式 | 翻阅命令历史 |
| `↑` / `↓` | 命令面板 | 选择面板中的命令 |
| `Tab` | 命令面板 | 自动补全 |
| `PgUp` / `PgDn` | 任意 | 输出区上下翻页 |
| `End` | 任意 | 输出区回到最新 |
| `Esc` | 任意 | 关闭面板 / 清空输入 |
| `Enter` | 任意 | 执行当前命令 |
| `Backspace` | 任意 | 删除输入字符 |
| `q` | 输入为空时 | 退出程序 |

---

## 🖥️ 界面结构

```
 orange-cli v0.1.0                                  11 项
──────────────────────────────────────────────────────────
 ● orange-cli 已就绪
 ● 输入 / 浏览命令  |  ↑↓ 翻阅历史  |  Tab 自动补全
 ● 已加载 11 个命令

┌ 命令 ──────────────────────────────────────────────────┐
│ ▶ /download   下载视频 (yt-dlp + aria2c)               │
│   /info       查看媒体文件信息 (ffprobe)                │
│   /play       播放视频/音频 (ffplay)                    │
│   /convert    视频格式转换 (ffmpeg)                     │
│   ...                                                  │
└────────────────────────────────────────────────────────┘
▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
 ▎ /download █
  / 命令  ↑↓ 历史  Tab 补全  Esc 清空  Enter 执行
```

- **Header** — 应用名 + 版本 + 命令数量
- **Output** — 命令执行结果，`●` 颜色区分类型（蓝=信息 / 绿=成功 / 橙=警告 / 红=错误）
- **Palette** — 命令面板，`▶` 标记当前选中项
- **Input** — `▎` 橙色输入提示符 + `█` 光标

---

## 🔧 前置依赖

程序启动时会按以下顺序查找外部工具：

1. **`assets/` 目录**（与 exe 同级或项目根目录）— 优先使用
2. **系统 PATH** — 回退方案

> `assets/` 已包含：`yt-dlp.exe` / `aria2c.exe` / `orange-cli.exe`
> 
> FFmpeg 二进制（`ffmpeg.exe` / `ffplay.exe` / `ffprobe.exe`）因超过 GitHub 100MB 限制未包含在仓库中，请从 [ffmpeg.org](https://ffmpeg.org/download.html) 下载后放入 `assets/`，或通过包管理器安装到 PATH。

### 手动安装依赖

```bash
# Windows (scoop)
scoop install ffmpeg yt-dlp aria2

# macOS
brew install ffmpeg yt-dlp aria2

# Linux (Debian/Ubuntu)
sudo apt install ffmpeg yt-dlp aria2
```

---

## 🚀 快速开始

```bash
# 1. 克隆仓库
git clone https://github.com/Orange-Git-Coder/orange-ffmpeg.git
cd orange-ffmpeg

# 2. 编译 Release（需要 Rust 1.80+）
cargo build --release

# 3. 运行
./target/release/orange-cli.exe          # Windows
./target/release/orange-cli             # macOS / Linux
```

> 也可以直接运行 `assets/orange-cli.exe`（已预编译，依赖 `assets/` 中的工具）

---

## 🛠️ 技术栈

| 组件 | 用途 |
|------|------|
| [Rust](https://www.rust-lang.org) | 语言 — 安全、零成本抽象 |
| [ratatui](https://ratatui.rs) | TUI 框架 — 终端界面渲染 |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 终端控制 — raw mode、事件处理 |
| [tokio](https://tokio.rs) | 异步运行时 — 后续异步命令执行 |
| [serde](https://serde.rs) / [toml](https://toml.io) | 配置序列化 |
| [anyhow](https://github.com/dtolnay/anyhow) | 错误处理 |

---

## 📁 项目结构

```
orange-cli/
├── assets/                # 外部工具二进制
│   ├── yt-dlp.exe         # ✅ 已包含
│   ├── aria2c.exe         # ✅ 已包含
│   ├── orange-cli.exe     # ✅ 预编译主程序
│   ├── ffmpeg.exe         # ⚠️ 需自行下载 (GitHub 100MB限制)
│   ├── ffplay.exe         # ⚠️ 需自行下载
│   └── ffprobe.exe        # ⚠️ 需自行下载
├── src/
│   ├── main.rs           # 入口 — 终端初始化
│   ├── app.rs            # 事件循环 + 按键处理
│   ├── state.rs          # 全局状态（输入/历史/输出/面板）
│   ├── config.rs         # 配置加载 + 工具路径解析
│   ├── commands/         # 命令实现
│   │   ├── mod.rs        # Command trait + Registry
│   │   ├── download.rs   # /download
│   │   ├── info.rs       # /info
│   │   ├── play.rs       # /play
│   │   ├── convert.rs    # /convert
│   │   ├── compress.rs   # /compress
│   │   ├── gif.rs        # /gif
│   │   ├── audio.rs      # /audio
│   │   ├── merge.rs      # /merge
│   │   ├── subtitle.rs   # /subtitle
│   │   ├── screenshot.rs # /screenshot
│   │   └── aria2.rs      # /aria2
│   ├── tools/            # 外部工具封装
│   │   ├── ffmpeg.rs
│   │   ├── ffplay.rs
│   │   ├── ffprobe.rs
│   │   ├── ytdlp.rs
│   │   └── aria2.rs
│   └── ui/               # 终端界面
│       ├── mod.rs        # 布局引擎
│       ├── colors.rs     # 配色方案
│       ├── header.rs     # 顶部状态栏
│       ├── output.rs     # 输出日志区
│       ├── palette.rs    # 命令面板
│       └── command_bar.rs # 输入栏
├── Cargo.toml
└── README.md
```

---

## ⚙️ 配置文件

首次运行后自动生成默认配置：

| 平台 | 路径 |
|------|------|
| Windows | `C:\Users\<用户名>\AppData\Roaming\orange-cli\orange-cli.toml` |
| macOS | `~/Library/Application Support/orange-cli/orange-cli.toml` |
| Linux | `~/.config/orange-cli/orange-cli.toml` |

```toml
download_dir = "./downloads"
ffmpeg_path = "ffmpeg"       # 自动优先使用 assets/ 中的二进制
ffprobe_path = "ffprobe"
ffplay_path = "ffplay"
ytdlp_path = "yt-dlp"
aria2_path = "aria2c"
aria2_max_connections = 16
aria2_split = 16
default_crf = "23"
default_preset = "medium"
```

---

## 📄 License

MIT © Orange-Git-Coder
