use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::AppState;
use super::colors;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    if area.height == 0 {
        return;
    }

    // ── Hint when typing / but palette not fully open ──────────────
    if !state.palette_open || state.palette_filtered.is_empty() {
        if !state.input.is_empty() && state.input.starts_with('/') {
            let hint = if state.palette_filtered.is_empty() {
                Span::styled(
                    "  (没有匹配的命令)",
                    Style::default().fg(colors::TEXT_MUTED),
                )
            } else {
                let idx = state.palette_filtered[0];
                let best = &state.palette_items[idx];
                Span::styled(
                    format!("  {} → {}", best.name, best.description),
                    Style::default().fg(colors::TEXT_MUTED),
                )
            };
            f.render_widget(Paragraph::new(Line::from(hint)), area);
        }
        return;
    }

    // ── Palette list ────────────────────────────────────────────────
    let inner_h = area.height.saturating_sub(2);
    let max_show = inner_h.min(state.palette_filtered.len() as u16) as usize;
    if max_show == 0 {
        return;
    }

    let lines: Vec<Line> = state
        .palette_filtered
        .iter()
        .take(max_show)
        .enumerate()
        .map(|(i, &idx)| {
            let item = &state.palette_items[idx];
            let is_selected = i == state.palette_selected;

            let (cursor, name_style, desc_style) = if is_selected {
                (
                    "▶",
                    Style::default()
                        .fg(colors::ORANGE)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(colors::TEXT_SECONDARY),
                )
            } else {
                (
                    " ",
                    Style::default()
                        .fg(colors::TEXT_PRIMARY)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(colors::TEXT_MUTED),
                )
            };

            Line::from(vec![
                Span::styled(
                    format!("  {} ", cursor),
                    Style::default().fg(colors::ORANGE),
                ),
                Span::styled(&item.name, name_style),
                Span::styled(format!("  {}", item.description), desc_style),
            ])
        })
        .collect();

    let count = state.palette_filtered.len();
    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(colors::BORDER))
        .title(Span::styled(
            format!(" 命令 ({}) ", count),
            Style::default().fg(colors::TEXT_MUTED),
        ))
        .style(Style::default().bg(colors::BG_PANEL));

    f.render_widget(Paragraph::new(lines).block(block), area);
}
