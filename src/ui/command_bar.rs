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
    if area.height < 3 {
        return;
    }

    // ── Divider ────────────────────────────────────────────────────
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "▔".repeat(area.width as usize),
            Style::default().fg(colors::DIVIDER),
        ))),
        Rect {
            y: area.y,
            height: 1,
            ..area
        },
    );

    // ── Input ──────────────────────────────────────────────────────
    let input_area = Rect {
        y: area.y + 1,
        height: 1,
        ..area
    };
    let input_text = if state.input.is_empty() {
        Span::styled("输入命令…", Style::default().fg(colors::TEXT_HINT))
    } else {
        Span::styled(&state.input, Style::default().fg(colors::TEXT))
    };

    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled("▎", Style::default().fg(colors::ORANGE)),
            Span::styled(" ", Style::default()),
            input_text,
            Span::styled("█", Style::default().fg(colors::ORANGE)),
        ])),
        input_area,
    );

    // ── Hint ───────────────────────────────────────────────────────
    let hint_area = Rect {
        y: area.y + 2,
        height: 1,
        ..area
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "  / 命令  ↑↓ 历史  Tab 补全  Esc 清空  Enter 执行",
            Style::default().fg(colors::TEXT_HINT),
        ))),
        hint_area,
    );
}
