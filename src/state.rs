use std::collections::VecDeque;

// ── Output Line ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub kind: OutputKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    Info,
    Success,
    Warn,
    Error,
    Command,
    Result,
}

impl OutputKind {
    pub fn prefix(&self) -> &str {
        match self {
            OutputKind::Info => "ℹ",
            OutputKind::Success => "✓",
            OutputKind::Warn => "⚠",
            OutputKind::Error => "✖",
            OutputKind::Command => "›",
            OutputKind::Result => "·",
        }
    }
}

// ── Command Palette Item ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PaletteItem {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
}

// ── Input Mode ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,     // Typing a command
    Palette,    // Command palette open
    SubCommand, // In a sub-command context (e.g., /convert)
}

// ── AppState ─────────────────────────────────────────────────────────

pub struct AppState {
    pub should_quit: bool,

    // Command input
    pub input: String,
    pub cursor_pos: usize,
    pub input_mode: InputMode,

    // History
    pub history: VecDeque<String>,
    pub history_idx: Option<usize>,

    // Command palette
    pub palette_items: Vec<PaletteItem>,
    pub palette_filtered: Vec<usize>,
    pub palette_selected: usize,
    pub palette_scroll: usize,
    pub palette_open: bool,

    // Output window
    pub output_lines: VecDeque<OutputLine>,
    pub output_scroll: usize,

    // Current command context
    pub active_command: Option<String>,
    pub command_args: Vec<String>,

    // Stats
    pub tick_count: u64,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            should_quit: false,

            input: String::new(),
            cursor_pos: 0,
            input_mode: InputMode::Normal,

            history: VecDeque::new(),
            history_idx: None,

            palette_items: Vec::new(),
            palette_filtered: Vec::new(),
            palette_selected: 0,
            palette_scroll: 0,
            palette_open: false,

            output_lines: VecDeque::new(),
            output_scroll: usize::MAX, // auto-bottom

            active_command: None,
            command_args: Vec::new(),

            tick_count: 0,
        }
    }

    pub fn push_output(&mut self, kind: OutputKind, text: &str) {
        // If user has manually scrolled up, keep their position;
        // otherwise auto-scroll to bottom.
        let was_at_bottom = self.output_scroll == usize::MAX;

        self.output_lines.push_back(OutputLine {
            text: text.to_string(),
            kind,
        });
        if self.output_lines.len() > 1000 {
            self.output_lines.pop_front();
        }

        if was_at_bottom || self.output_lines.len() <= 1 {
            self.output_scroll = usize::MAX; // auto-bottom
        }
    }

    pub fn push_history(&mut self, cmd: &str) {
        if self.history.back().map(|s| s.as_str()) != Some(cmd) {
            self.history.push_back(cmd.to_string());
            if self.history.len() > 200 {
                self.history.pop_front();
            }
        }
        self.history_idx = None;
    }

    pub fn history_up(&mut self) -> Option<String> {
        if self.history.is_empty() {
            return None;
        }
        let idx = match self.history_idx {
            Some(i) if i > 0 => i - 1,
            _ => self.history.len() - 1,
        };
        self.history_idx = Some(idx);
        self.history.get(idx).cloned()
    }

    pub fn history_down(&mut self) -> Option<String> {
        match self.history_idx {
            Some(i) if i + 1 < self.history.len() => {
                self.history_idx = Some(i + 1);
                self.history.get(i + 1).cloned()
            }
            Some(_) => {
                self.history_idx = None;
                None
            }
            None => None,
        }
    }
}
