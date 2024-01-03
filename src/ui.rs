use crate::GameState;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use std::io;

pub fn ui_render(f: &mut Frame<CrosstermBackend<io::Stdout>>, state: &mut GameState) {
    let size = f.size();

    // MAIN BLOCK
    let main_block = Block::default()
        .title("Tic Tac Toe")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick)
        .border_style(Style::default().fg(Color::White));

    let area = centered_rect(30, 55, size, f, state);

    // Game board
    game_area_render(f, main_block.inner(area), state);

    f.render_widget(main_block, area);
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: Rect,
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    let top_text = if state.winner != ' ' {
        format!("The winner is player {}", state.winner)
    } else if state.case_left == 0 {
        "It's a draw".to_owned()
    } else {
        format!("Player's turn: {}", state.next_player)
    };

    let bottom_text = if state.is_game_end() {
        vec![
            Spans::from(Span::raw("Play again: r")),
            Spans::from(Span::raw("Quit: q")),
        ]
    } else {
        vec![
            Spans::from(Span::raw("Movement: ← ↓ ↑ →")),
            Spans::from(Span::raw("Claim a box: ENTER / SPACE")),
            Spans::from(Span::raw("Quit: q")),
        ]
    };
    let top_text = std::iter::repeat(Spans::from(Span::raw("")))
        .take(10)
        .chain(std::iter::once(Spans::from(Span::raw(top_text))))
        .collect::<Vec<_>>();

    let player_turn_paragraph =
        Paragraph::new(top_text)
            .alignment(Alignment::Center)
            .style(Style::default().add_modifier(if state.is_game_end() {
                Modifier::RAPID_BLINK
            } else {
                Modifier::BOLD
            }));

    f.render_widget(player_turn_paragraph, popup_layout[0]);

    let helper_paragraph = Paragraph::new(bottom_text).alignment(Alignment::Center);

    f.render_widget(helper_paragraph, popup_layout[2]);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn game_area_render(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    r: Rect,
    state: &mut GameState,
) {
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(r.height / 3); 3])
        .split(r);

    for (row_index, row) in layout_vertical.iter().enumerate() {
        let layout_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(row.width / 3); 3])
            .split(*row);

        for (slot_index, slot) in layout_horizontal.iter().enumerate() {
            let current_case_char = state.board[row_index][slot_index];
            let mut box_color = Color::LightYellow;

            if row_index == state.cursor_y as usize && slot_index == state.cursor_x as usize {
                box_color = Color::White;
            }

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(box_color));

            let text_case = state.ascii_current_case(current_case_char);

            let text_case_widget = Paragraph::new(text_case)
                .block(block.clone())
                .alignment(Alignment::Center)
                .style(Style::default().fg(if current_case_char == 'X' {
                    Color::LightGreen
                } else {
                    Color::LightRed
                }));
            f.render_widget(text_case_widget, *slot);
            f.render_widget(block, *slot);
        }
    }
}
