#![allow(dead_code)]

use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::commands::CommandRegistry;
use crate::config::AppConfig;
use crate::state::{AppState, InputMode, OutputKind};
use crate::ui;

pub struct App {
    state: AppState,
    registry: CommandRegistry,
}

impl App {
    pub fn new() -> Self {
        let _config = AppConfig::load().unwrap_or_default();
        let mut state = AppState::new();
        let registry = CommandRegistry::new();

        // Populate palette items from registry
        state.palette_items = registry
            .list_all()
            .iter()
            .map(|c| crate::state::PaletteItem {
                name: c.name().to_string(),
                description: c.description().to_string(),
                aliases: c.aliases().iter().map(|a| a.to_string()).collect(),
            })
            .collect();

        state.push_output(OutputKind::Success, "🍊 orange-cli 已就绪");
        state.push_output(
            OutputKind::Info,
            "输入 / 浏览命令  |  ↑↓ 翻阅历史  |  Tab 自动补全",
        );
        state.push_output(
            OutputKind::Info,
            &format!("已加载 {} 个命令", state.palette_items.len()),
        );

        App { state, registry }
    }

    pub fn run_event_loop(
        terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        let mut app = App::new();

        let tick_rate = Duration::from_millis(50);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| ui::render(f, &mut app.state))?;

            if app.state.should_quit {
                break;
            }

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(Duration::ZERO);

            if event::poll(timeout)? {
                loop {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            app.handle_key(key.code);
                        }
                    }
                    if !event::poll(Duration::ZERO)? {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.state.tick_count += 1;
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    // ── Key Handler ──────────────────────────────────────────────────

    fn handle_key(&mut self, key: KeyCode) {
        // Global keys: output scrolling works in any mode
        match key {
            KeyCode::PageUp => {
                self.scroll_output_up();
                return;
            }
            KeyCode::PageDown => {
                self.scroll_output_down();
                return;
            }
            KeyCode::End => {
                self.state.output_scroll = usize::MAX;
                return;
            }
            _ => {}
        }

        match self.state.input_mode {
            InputMode::Normal => self.handle_normal_key(key),
            InputMode::Palette => self.handle_palette_key(key),
            InputMode::SubCommand => self.handle_normal_key(key),
        }
    }

    fn scroll_output_up(&mut self) {
        let visible_h = 10; // rough estimate; clamped in render anyway
        let total = self.state.output_lines.len();
        let max_scroll = total.saturating_sub(visible_h);
        let cur = if self.state.output_scroll == usize::MAX {
            max_scroll
        } else {
            self.state.output_scroll.min(max_scroll)
        };
        self.state.output_scroll = cur.saturating_sub(5);
    }

    fn scroll_output_down(&mut self) {
        let visible_h = 10;
        let total = self.state.output_lines.len();
        let max_scroll = total.saturating_sub(visible_h);
        let cur = if self.state.output_scroll == usize::MAX {
            max_scroll
        } else {
            self.state.output_scroll.min(max_scroll)
        };
        let new_scroll = cur.saturating_add(5);
        if new_scroll >= max_scroll {
            self.state.output_scroll = usize::MAX; // snap to bottom
        } else {
            self.state.output_scroll = new_scroll;
        }
    }

    fn handle_normal_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') if self.state.input.is_empty() => {
                self.state.push_output(OutputKind::Info, "Goodbye!");
                self.state.should_quit = true;
                return;
            }
            KeyCode::Esc => {
                self.state.input.clear();
                self.state.palette_open = false;
                self.state.palette_filtered.clear();
                self.state.palette_selected = 0;
                return;
            }
            KeyCode::Enter => {
                self.execute_input();
                return;
            }
            KeyCode::Tab => {
                self.try_autocomplete();
                return;
            }
            KeyCode::Up => {
                if let Some(hist) = self.state.history_up() {
                    self.state.input = hist;
                }
                return;
            }
            KeyCode::Down => {
                if let Some(hist) = self.state.history_down() {
                    self.state.input = hist;
                } else {
                    self.state.input.clear();
                }
                return;
            }
            KeyCode::Backspace => {
                self.state.input.pop();
                if self.state.input.is_empty() {
                    self.state.palette_open = false;
                }
            }
            KeyCode::Char(c) => {
                self.state.input.push(c);
            }
            _ => {}
        }

        // After any input change, update palette filtering
        self.update_palette();
    }

    fn handle_palette_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.state.palette_open = false;
                if self.state.input.is_empty() {
                    self.state.palette_filtered.clear();
                }
            }
            KeyCode::Enter => {
                // Select highlighted palette item
                if !self.state.palette_filtered.is_empty() {
                    let idx = self.state.palette_filtered[self.state.palette_selected];
                    let name = self.state.palette_items[idx].name.clone();
                    self.state.input = name;
                    self.state.palette_open = false;
                    self.execute_input();
                    return;
                }
            }
            KeyCode::Up => {
                if self.state.palette_selected > 0 {
                    self.state.palette_selected -= 1;
                    // Scroll up if selection goes above visible area
                    if self.state.palette_selected < self.state.palette_scroll {
                        self.state.palette_scroll = self.state.palette_selected;
                    }
                }
            }
            KeyCode::Down => {
                if self.state.palette_selected + 1 < self.state.palette_filtered.len() {
                    self.state.palette_selected += 1;
                    // Scroll down if selection goes below visible area
                    // visible_h is dynamic; use a generous estimate, clamped in render
                    let visible_end = self.state.palette_scroll.saturating_add(8);
                    if self.state.palette_selected >= visible_end {
                        self.state.palette_scroll = self.state.palette_selected.saturating_sub(4);
                    }
                }
            }
            KeyCode::Tab => {
                // Autocomplete the top match
                if !self.state.palette_filtered.is_empty() {
                    let idx = self.state.palette_filtered[0];
                    let name = &self.state.palette_items[idx].name;
                    self.state.input = name.clone();
                    self.state.palette_open = false;
                }
            }
            KeyCode::Backspace => {
                self.state.input.pop();
                self.state.palette_selected = 0;
                if self.state.input.is_empty() {
                    self.state.palette_open = false;
                }
            }
            KeyCode::Char(c) => {
                self.state.input.push(c);
                self.state.palette_selected = 0;
            }
            _ => {}
        }
        self.update_palette();
    }

    // ── Palette ──────────────────────────────────────────────────────

    fn update_palette(&mut self) {
        if !self.state.input.starts_with('/') {
            self.state.palette_open = false;
            self.state.input_mode = InputMode::Normal;
            return;
        }

        let matches = self.registry.fuzzy_search(&self.state.input);
        self.state.palette_filtered = matches
            .iter()
            .filter_map(|c| {
                self.state
                    .palette_items
                    .iter()
                    .position(|p| p.name == c.name())
            })
            .collect();

        self.state.palette_open = !self.state.palette_filtered.is_empty();
        if self.state.palette_open {
            self.state.input_mode = InputMode::Palette;
        } else {
            self.state.input_mode = InputMode::Normal;
        }
        if self.state.palette_selected >= self.state.palette_filtered.len() {
            self.state.palette_selected = 0;
        }
        self.state.palette_scroll = 0;
    }

    fn try_autocomplete(&mut self) {
        if !self.state.input.starts_with('/') {
            return;
        }

        let completions = self.registry.autocomplete(&self.state.input);
        if completions.len() == 1 {
            self.state.input = completions[0].clone();
            self.state.palette_open = false;
        } else if completions.len() > 1 {
            // Open palette with completions
            self.update_palette();
        }
    }

    // ── Execution ────────────────────────────────────────────────────

    fn execute_input(&mut self) {
        let input = self.state.input.trim().to_string();
        if input.is_empty() {
            return;
        }

        self.state.push_history(&input);

        // Parse: /command arg1 arg2 ...
        let parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        let cmd_name = &parts[0];
        let args: Vec<String> = if parts.len() > 1 {
            parts[1..].to_vec()
        } else {
            vec![]
        };

        if !cmd_name.starts_with('/') {
            self.state
                .push_output(OutputKind::Warn, "命令必须以 / 开头. 输入 / 打开命令面板");
            self.state.input.clear();
            return;
        }

        match self.registry.find(cmd_name) {
            Some(cmd) => match cmd.execute(&mut self.state, &args) {
                Ok(()) => {}
                Err(e) => {
                    self.state
                        .push_output(OutputKind::Error, &format!("执行失败: {}", e));
                }
            },
            None => {
                self.state
                    .push_output(OutputKind::Error, &format!("未知命令: {}", cmd_name));
                self.state
                    .push_output(OutputKind::Info, "输入 /? 查看所有可用命令");
            }
        }

        self.state.input.clear();
        self.state.palette_open = false;
        self.state.input_mode = InputMode::Normal;
    }
}
