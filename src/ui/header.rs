use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::colors;
use crate::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    // Left: logo + app name
    let left = vec![
        Span::styled(" 🍊", Style::default().fg(colors::ORANGE)),
        Span::styled(" orange-cli", Style::default().fg(colors::TEXT_PRIMARY)),
        Span::styled(
            format!(" v{}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(colors::TEXT_MUTED),
        ),
    ];

    // Right: status
    let cmd_count = state.palette_items.len();
    let mode_str = match state.input_mode {
        crate::state::InputMode::Normal if state.input.is_empty() => "",
        crate::state::InputMode::Normal => "⌨",
        crate::state::InputMode::Palette => "🔍",
        crate::state::InputMode::SubCommand => "⚙",
    };
    let right = vec![Span::styled(
        format!("{} 项命令 | {}", cmd_count, mode_str),
        Style::default().fg(colors::TEXT_MUTED),
    )];

    // Layout: left … padding … right
    let left_width: usize = left.iter().map(|s| s.content.len()).sum();
    let right_width: usize = right.iter().map(|s| s.content.len()).sum();
    let padding = (area.width as usize).saturating_sub(left_width + right_width);

    let mut spans: Vec<Span> = left;
    spans.push(Span::styled(
        " ".repeat(padding),
        Style::default().bg(colors::BG_DARK),
    ));
    spans.extend(right);

    f.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(colors::BG_DARK)),
        area,
    );
}
