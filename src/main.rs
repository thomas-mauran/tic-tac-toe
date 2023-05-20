use std::{io::{self}};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, KeyCode, Event, self}, style::Colors};
use ratatui::{backend::{CrosstermBackend}, Terminal, Frame, layout::{Layout, Direction, Constraint, Rect, Alignment}, widgets::{Block, Borders, Paragraph, BorderType}, style::{ Style, Color, Modifier}, text::{Spans, Span}};



struct GameState{
    cursor_x: usize,
    cursor_y: usize,
    next_player: char,
    board: Vec<Vec<char>>,
    case_left: usize,
    winner: char,
}

impl Default for GameState{
    fn default() -> Self{

        let board = vec![
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
        ];

        GameState { 
            cursor_x: 0, 
            cursor_y: 0,
            next_player: 'X',
            board: board,
            case_left: 9,
            winner: ' ',
            }
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
    fn select_case(&mut self){
        if self.board[self.cursor_y][self.cursor_x] == ' '{
            self.board[self.cursor_y][self.cursor_x] = self.next_player;
            self.check_win();
        }
    }

    fn switch_next_player(&mut self){
        match self.next_player{
            'X' => self.next_player = 'O',
            _ => self.next_player = 'X'
        }
    }

    fn ascii_current_case(&mut self, character: char) -> Vec<Spans>{

        match character{
            'O' => return vec![
                Spans::from(Span::raw("   ____  ")),
                Spans::from(Span::raw("  / __ \\")),
                Spans::from(Span::raw(" | |  | |")),
                Spans::from(Span::raw(" | |  | |")),
                Spans::from(Span::raw(" | |__| |")),
                Spans::from(Span::raw("  \\____/ ")),
            ],
         
            'X'=> vec![
                Spans::from(Span::raw("  __   __  ")),
                Spans::from(Span::raw(" \\ \\ / /")),
                Spans::from(Span::raw("  \\ V / ")),
                Spans::from(Span::raw("   > <  ")),
                Spans::from(Span::raw("  / . \\ ")),
                Spans::from(Span::raw(" /_/ \\_\\"))],
            _ => vec![
                Spans::from(Span::raw("        ")),
                Spans::from(Span::raw("        ")),
                Spans::from(Span::raw("        ")),
                Spans::from(Span::raw("        ")),
                Spans::from(Span::raw("        ")),
                Spans::from(Span::raw("        "))
            ],

        }
    }

    fn check_win(&mut self){
        self.case_left -= 1;
        // check row win
        for row in 0..3{
            if self.board[row][0] != ' ' && self.board[row][0] == self.board[row][1] && self.board[row][1] == self.board[row][2]{
                self.winner = self.board[row][0];
            }
        }

        // check col win
        for col in 0..3{
            if self.board[0][col] != ' ' && self.board[0][col] == self.board[1][col] && self.board[1][col] == self.board[2][col]{
                self.winner = self.board[0][col];
            }
        }

        // check for diagonal win
        if self.board[0][0] != ' ' && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] {
            self.winner = self.board[0][0];   
        }
        // check for diagonal win other side
        if self.board[0][2] != ' ' && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            self.winner = self.board[0][2];   
        }
        // else we switch the player 
        else{
            self.switch_next_player();
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
                KeyCode::Enter | KeyCode::Char(' ') => state.select_case(),
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
    .border_type(ratatui::widgets::BorderType::Thick)
    .border_style(Style::default().fg(Color::White));

    let area = centered_rect(30, 55, size, f,state);

    // Game board
    game_area_render(f, main_block.inner(area), state);

    f.render_widget(main_block, area);

}


/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect, f: &mut Frame<CrosstermBackend<io::Stdout>>, state: &mut GameState) -> Rect {
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

    let mut text = format!("Player's turn: {}", state.next_player);
    // Top text
    if(state.winner != ' '){
        text = format!("The winner is player {}", state.winner)
    }

    if(state.winner == ' ' && state.case_left == 0){
        text = "It's a draw".to_owned()
    }
    let player_text = std::iter::repeat(Spans::from(Span::raw("")))
    .take(10)
    .chain(std::iter::once(Spans::from(Span::raw(text))))
    .collect::<Vec<_>>();

    let player_turn_paragraph = Paragraph::new(player_text)
        .alignment(Alignment::Center).style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_widget(player_turn_paragraph, popup_layout[0]);

    // Help text
    let helper_text: Vec<Spans> = vec![
        Spans::from(Span::raw("Movement: ← ↓ ↑ →")),
        Spans::from(Span::raw("Claim a box: ENTER / SPACE")),
        Spans::from(Span::raw("Quit: q")),
    ];
    let helper_paragraph = Paragraph::new(helper_text)
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
                let current_case_char = state.board[chunk_index][slot_index];
                let mut box_color = Color::LightYellow;       

                if chunk_index == state.cursor_y as usize && slot_index == state.cursor_x as usize {
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
                    .style(Style::default().fg(if current_case_char == 'X' { Color::LightGreen } else { Color::LightRed }));
                f.render_widget(text_case_widget, *slot);
                f.render_widget(block, *slot);
            }
        }
    }
}

