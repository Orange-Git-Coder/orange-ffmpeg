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
    let term_h = area.height;

    let palette_open = state.palette_open && !state.palette_filtered.is_empty();
    let input_h = 3u16; // divider + input + hint

    // When palette is open, give it generous space and cap output
    let (output_c, palette_h) = if palette_open {
        let max_palette = term_h.saturating_sub(1 + 6 + input_h); // header + min output + input
        let needed = (state.palette_filtered.len() as u16).saturating_add(2); // items + border
        let h = needed.min(max_palette).max(3);
        (Constraint::Length(6), h) // output capped to 6 lines
    } else if !state.input.is_empty() && state.input.starts_with('/') {
        (Constraint::Min(3), 1u16) // hint only
    } else {
        (Constraint::Min(3), 0u16)
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),         // header
            output_c,                      // output (shrinks when palette open)
            Constraint::Length(palette_h), // palette
            Constraint::Length(input_h),   // input bar
        ])
        .split(area);

    header::render(f, chunks[0], state);
    output::render(f, chunks[1], state);
    palette::render(f, chunks[2], state);
    command_bar::render(f, chunks[3], state);
}
