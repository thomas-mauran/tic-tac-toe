use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game_state::GameState;
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io::{self};
use ui::ui_render;
mod game_state;
mod ui;

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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut GameState,
) -> io::Result<()> {
    loop {
        // Draw the ui
        terminal.draw(|f: &mut Frame| ui_render(f, state))?;

        // Catch inputs
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        return Ok(());
                    }
                    KeyCode::Left => state.move_horizontal(-1),
                    KeyCode::Right => state.move_horizontal(1),
                    KeyCode::Up => state.move_vertical(-1),
                    KeyCode::Down => state.move_vertical(1),
                    KeyCode::Enter | KeyCode::Char(' ') => state.select_case(),
                    KeyCode::Char('r') => state.reload_game(),
                    _ => {}
                }
            }
        }
    }
}
