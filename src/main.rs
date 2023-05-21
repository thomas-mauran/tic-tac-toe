use std::{io::{self}};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}};
use ratatui::{backend::{CrosstermBackend}, Terminal, Frame};
use ui::ui_render;
use game_state::GameState;
mod ui;
mod game_state;

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
            ui_render(f, state)
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
                KeyCode::Enter | KeyCode::Char(' ') => state.select_case(),
                KeyCode::Char('r') => state.reload_game(),
                _ => {},

            }

        }

    }
}




