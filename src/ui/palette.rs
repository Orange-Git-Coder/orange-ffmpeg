use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::colors;
use crate::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    if area.height == 0 {
        return;
    }

    // ── Hint when no full palette ──────────────────────────────────
    if !state.palette_open || state.palette_filtered.is_empty() {
        if !state.input.is_empty() && state.input.starts_with('/') {
            if state.palette_filtered.is_empty() {
                f.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        "  无匹配",
                        Style::default().fg(colors::TEXT_DIM),
                    ))),
                    area,
                );
            } else {
                let idx = state.palette_filtered[0];
                let best = &state.palette_items[idx];
                let s = format!("  {} → {}", best.name, best.description);
                f.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        s,
                        Style::default().fg(colors::TEXT_DIM),
                    ))),
                    area,
                );
            }
        }
        return;
    }

    // ── List ───────────────────────────────────────────────────────
    let inner_h = area.height.saturating_sub(2);
    let max_show = inner_h.min(state.palette_filtered.len() as u16) as usize;
    if max_show == 0 {
        return;
    }

    let scroll = state
        .palette_scroll
        .min(state.palette_filtered.len().saturating_sub(max_show));

    let lines: Vec<Line> = state
        .palette_filtered
        .iter()
        .skip(scroll)
        .take(max_show)
        .enumerate()
        .map(|(i, &idx)| {
            let item = &state.palette_items[idx];
            let real_idx = scroll + i;
            let selected = real_idx == state.palette_selected;

            let (prefix, s, s_bold, s_dim) = if selected {
                (
                    "▶",
                    Style::default().fg(colors::ORANGE),
                    Style::default().fg(colors::ORANGE).add_modifier(Modifier::BOLD),
                    Style::default().fg(colors::ORANGE_LIGHT),
                )
            } else {
                (
                    " ",
                    Style::default().fg(colors::TEXT),
                    Style::default().fg(colors::TEXT).add_modifier(Modifier::BOLD),
                    Style::default().fg(colors::TEXT_DIM),
                )
            };

            Line::from(vec![
                Span::styled(format!(" {} ", prefix), s),
                Span::styled(&item.name, s_bold),
                Span::styled(format!("  {}", item.description), s_dim),
            ])
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors::BORDER))
        .title(Span::styled(" 命令 ", Style::default().fg(colors::TEXT_DIM)));

    f.render_widget(Paragraph::new(lines).block(block), area);
}
