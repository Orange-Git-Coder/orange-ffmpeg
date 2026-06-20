use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
    Frame,
};

use super::colors;
use crate::state::{AppState, OutputKind};

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let visible_h = area.height as usize;
    if visible_h == 0 {
        return;
    }

    let total = state.output_lines.len();
    let max_scroll = total.saturating_sub(visible_h);
    let scroll = if state.output_scroll == usize::MAX {
        max_scroll
    } else {
        state.output_scroll.min(max_scroll)
    };

    let mut lines: Vec<Line> = state
        .output_lines
        .iter()
        .skip(scroll)
        .take(visible_h)
        .map(|ol| {
            let (icon, color) = match ol.kind {
                OutputKind::Info => ("●", colors::INFO),
                OutputKind::Success => ("●", colors::SUCCESS),
                OutputKind::Warn => ("●", colors::WARN),
                OutputKind::Error => ("●", colors::ERROR),
                OutputKind::Command => ("●", colors::COMMAND),
                OutputKind::Result => (" ", colors::TEXT),
            };
            Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(&ol.text, Style::default().fg(colors::TEXT)),
            ])
        })
        .collect();

    // Scroll indicator
    if scroll < max_scroll {
        lines.push(Line::from(Span::styled(
            format!("  ↑ 已滚动 (PgUp/PgDn/End)"),
            Style::default().fg(colors::TEXT_HINT),
        )));
    }

    f.render_widget(Paragraph::new(Text::from(lines)), area);
}
