use std::{io::{self, Stdout}, thread, time::Duration};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::{Layout, Direction, Constraint, Rect, Alignment}, widgets::{Block, Borders, Clear, BorderType, Paragraph}, style::{Modifier, Style, Color}, text::{Spans, Span}};

fn main() -> Result<(), io::Error> {

    // Setup the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    run_app(&mut terminal)

}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()>{
    loop {
        // Draw the ui
        terminal.draw(|f: &mut Frame<CrosstermBackend<io::Stdout>>| {
            ui(f)
        })?;

        // Catch inputs
        if let Event::Key(key) = event::read()?{
            match key.code{
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
                    return Ok(());
                }
                KeyCode::Left => println!("Left"),
                KeyCode::Right => println!("Right"),
                KeyCode::Up => println!("Up"),
                KeyCode::Down => println!("Down"),
                _ => println!("other")

            }

        }

    }
}


fn ui(f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();

    // MAIN BLOCK
    let main_block = Block::default()
    .title("Tic Tac Toe")
    .borders(Borders::ALL)
    .border_type(tui::widgets::BorderType::Double)
    .border_style(Style::default().fg(Color::White));

    let area = centered_rect(30, 55, size, f);


    game_area_render(f, main_block.inner(area));

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

fn game_area_render(f: &mut Frame<CrosstermBackend<io::Stdout>>, r: Rect) {
    let layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(r.height / 3); 3])
        .split(r);

    for row in layout_vertical.chunks(3) {
        for chunk in row {
            let layout_horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(chunk.width / 3); 3])
                .split(*chunk);

            for slot in layout_horizontal.iter() {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::LightYellow));
                f.render_widget(block, *slot);
            }
        }
    }
}

