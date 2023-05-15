use std::{io::{self}};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}};
use tui::{backend::{CrosstermBackend}, Terminal, Frame, layout::{Layout, Direction, Constraint, Rect, Alignment}, widgets::{Block, Borders, Paragraph}, style::{ Style, Color, Modifier}, text::{Spans, Span}};


struct GameState{
    cursor_x: i32,
    cursor_y: i32,
    board: Vec<Vec<char>>
}

impl Default for GameState{
    fn default() -> Self{
        let cursor_x = 0;
        let cursor_y= 0;
        let board = vec![
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
        ];

        GameState { 
            cursor_x: cursor_x, 
            cursor_y: cursor_y, 
            board: board }
    }
}

impl GameState{
    fn move_horizontal(&mut self, value: i32){
        match value{
            -1 => {
                if self.cursor_x != 0{
                    self.cursor_x -= 1;
                }
            }
            1 => {
                if self.cursor_x != 2 {
                    self.cursor_x += 1;
                }
            }
            _ => panic!("Value must be -1 or 1")
        }

    }    
    fn move_vertical(&mut self, value: i32){
        match value{
            -1 => {
                if self.cursor_y != 0{
                    self.cursor_y -= 1;
                }
            }
            1 => {
                if self.cursor_y != 2 {
                    self.cursor_y += 1;
                }
            }
            _ => panic!("Value must be -1 or 1")
        }

    }    
}

fn main() -> Result<(), io::Error> {

    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Important variables
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut game_state = GameState::default();

    // App loop
    run_app(&mut terminal, &mut game_state)

}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &mut GameState) -> io::Result<()>{
    loop {
        // Draw the ui
        terminal.draw(|f: &mut Frame<CrosstermBackend<io::Stdout>>| {
            ui(f, state)
        })?;

        // Catch inputs
        if let Event::Key(key) = event::read()?{
            match key.code{
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                    return Ok(());
                }
                KeyCode::Left => state.move_horizontal(-1),
                KeyCode::Right => state.move_horizontal(1),
                KeyCode::Up => state.move_vertical(-1),
                KeyCode::Down => state.move_vertical(1),
                KeyCode::Enter | KeyCode::Char(' ') => println!("enter"),
                _ => println!("other")

            }

        }

    }
}



fn ui(f: &mut Frame<CrosstermBackend<io::Stdout>>, state: &mut GameState) {
    let size = f.size();

    // MAIN BLOCK
    let main_block = Block::default()
    .title("Tic Tac Toe")
    .borders(Borders::ALL)
    .border_type(tui::widgets::BorderType::Double)
    .border_style(Style::default().fg(Color::White));

    let area = centered_rect(30, 55, size, f);

    // Game board
    game_area_render(f, main_block.inner(area), state);

    f.render_widget(main_block, area);

}


/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect, f: &mut Frame<CrosstermBackend<io::Stdout>>) -> Rect {
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

    let text = vec![
        Spans::from(Span::raw("Movement: ← ↓ ↑ →")),
        Spans::from(Span::raw("Claim a box: ENTER / SPACE")),
        Spans::from(Span::raw("Quit: q")),
    ];
    let helper_paragraph = Paragraph::new(text)
        .alignment(Alignment::Center);
    
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

// Function to render the board 3x3
fn game_area_render(f: &mut Frame<CrosstermBackend<io::Stdout>>, r: Rect, state: &mut GameState) {
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(r.height / 3); 3])
        .split(r);

    for row in layout_vertical.chunks(3) {
        for (chunk_index, chunk) in row.iter().enumerate() {
            let layout_horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(chunk.width / 3); 3])
                .split(*chunk);

            for (slot_index, slot) in layout_horizontal.iter().enumerate() {
                let mut box_color = Color::LightYellow;
                let mut box_modifier = Modifier::HIDDEN;

                if chunk_index == state.cursor_y as usize && slot_index == state.cursor_x as usize {
                    box_color = Color::Cyan;
                    box_modifier = Modifier::RAPID_BLINK
                }

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(box_color).add_modifier(box_modifier));
                f.render_widget(block, *slot);
            }
        }
    }
}

