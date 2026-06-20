use ratatui::style::Color;

// ── orange-cli Color Palette (inspired by Claude Code) ──────────────
// Warm, modern, readable — optimized for dark terminal backgrounds.

// ── Primary / Accent ────────────────────────────────────────────────
pub const ORANGE: Color = Color::Rgb(255, 149, 0); // warm accent
pub const ORANGE_DIM: Color = Color::Rgb(200, 110, 0);
pub const AMBER: Color = Color::Rgb(255, 179, 0);

// ── Surfaces & Borders ──────────────────────────────────────────────
pub const BG_DARK: Color = Color::Rgb(18, 18, 18);
pub const BG_PANEL: Color = Color::Rgb(24, 24, 24);
pub const BORDER: Color = Color::Rgb(55, 55, 55);
pub const BORDER_ACTIVE: Color = Color::Rgb(80, 80, 80);
pub const DIVIDER: Color = Color::Rgb(45, 45, 45);

// ── Text ────────────────────────────────────────────────────────────
pub const TEXT_PRIMARY: Color = Color::Rgb(220, 220, 220);
pub const TEXT_SECONDARY: Color = Color::Rgb(160, 160, 160);
pub const TEXT_MUTED: Color = Color::Rgb(100, 100, 100);
pub const TEXT_HINT: Color = Color::Rgb(70, 70, 70);

// ── Semantic ────────────────────────────────────────────────────────
pub const INFO: Color = Color::Rgb(100, 180, 255);
pub const SUCCESS: Color = Color::Rgb(80, 200, 120);
pub const WARN: Color = Color::Rgb(255, 170, 50);
pub const ERROR: Color = Color::Rgb(255, 80, 80);
pub const COMMAND: Color = Color::Rgb(255, 179, 0);
pub const RESULT: Color = Color::Rgb(200, 200, 200);

// ── Selection ───────────────────────────────────────────────────────
pub const SELECTION_BG: Color = Color::Rgb(50, 50, 50);
pub const SELECTION_ACCENT: Color = Color::Rgb(255, 149, 0);
