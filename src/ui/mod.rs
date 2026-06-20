pub mod colors;
pub mod command_bar;
pub mod header;
pub mod output;
pub mod palette;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::state::AppState;

pub fn render(f: &mut Frame, state: &mut AppState) {
    let area = f.area();

    // Dynamic palette height
    let palette_h = if state.palette_open && !state.palette_filtered.is_empty() {
        (state.palette_filtered.len().min(10) + 2) as u16 // +2 for border
    } else if !state.input.is_empty() && state.input.starts_with('/') {
        1u16 // hint line
    } else {
        0u16
    };

    // Input bar is always 3 lines: divider + input + hint
    let input_h = 3u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),         // header
            Constraint::Min(3),            // output area
            Constraint::Length(palette_h), // palette (conditional)
            Constraint::Length(input_h),   // input bar (always visible)
        ])
        .split(area);

    header::render(f, chunks[0], state);
    output::render(f, chunks[1], state);
    palette::render(f, chunks[2], state);
    command_bar::render(f, chunks[3], state);
}
