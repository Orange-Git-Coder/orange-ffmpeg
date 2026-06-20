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
    let left = vec![
        Span::styled("orange-cli", Style::default().fg(colors::ORANGE)),
        Span::styled(
            format!(" v{}", env!("CARGO_PKG_VERSION")),
            Style::default().fg(colors::TEXT_DIM),
        ),
    ];

    let mode = match state.input_mode {
        crate::state::InputMode::Palette => "🔍",
        crate::state::InputMode::SubCommand => "⚙",
        _ => "",
    };
    let right = format!("{} 项", state.palette_items.len());
    let right_text = if mode.is_empty() {
        right
    } else {
        format!("{} {}", right, mode)
    };

    let left_w: usize = left.iter().map(|s| s.content.len()).sum();
    let pad = (area.width as usize).saturating_sub(left_w + right_text.len());

    let line = Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled(
            left.iter()
                .map(|s| s.content.clone())
                .collect::<Vec<_>>()
                .join(""),
            Style::default(),
        ),
        Span::styled(" ".repeat(pad), Style::default()),
        Span::styled(right_text, Style::default().fg(colors::TEXT_DIM)),
        Span::styled(" ", Style::default()),
    ]);

    f.render_widget(Paragraph::new(line), area);
}
