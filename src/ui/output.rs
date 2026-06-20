use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use crate::state::{AppState, OutputKind};
use super::colors;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let lines: Vec<Line> = state
        .output_lines
        .iter()
        .map(|ol| {
            let (icon, color) = match ol.kind {
                OutputKind::Info => ("ℹ", colors::INFO),
                OutputKind::Success => ("✓", colors::SUCCESS),
                OutputKind::Warn => ("⚠", colors::WARN),
                OutputKind::Error => ("✖", colors::ERROR),
                OutputKind::Command => ("›", colors::COMMAND),
                OutputKind::Result => ("·", colors::RESULT),
            };
            Line::from(vec![
                Span::styled(format!(" {} ", icon), Style::default().fg(color)),
                Span::styled(&ol.text, Style::default().fg(colors::TEXT_PRIMARY)),
            ])
        })
        .collect();

    f.render_widget(
        Paragraph::new(Text::from(lines)).style(Style::default().bg(colors::BG_DARK)),
        area,
    );
}
