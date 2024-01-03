use ratatui::text::{Span, Line};

pub struct GameState {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub next_player: char,
    pub board: Vec<Vec<char>>,
    pub case_left: usize,
    pub winner: char,
}

impl Default for GameState {
    fn default() -> Self {
        let board = vec![
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
        ];

        GameState {
            cursor_x: 0,
            cursor_y: 0,
            next_player: 'X',
            board,
            case_left: 9,
            winner: ' ',
        }
    }
}

impl GameState {
    pub fn move_horizontal(&mut self, value: i32) {
        if !self.is_game_end() {
            match value {
                -1 => {
                    if self.cursor_x != 0 {
                        self.cursor_x -= 1;
                    }
                }
                1 => {
                    if self.cursor_x != 2 {
                        self.cursor_x += 1;
                    }
                }
                _ => panic!("Value must be -1 or 1"),
            }
        }
    }
    pub fn move_vertical(&mut self, value: i32) {
        if !self.is_game_end() {
            match value {
                -1 => {
                    if self.cursor_y != 0 {
                        self.cursor_y -= 1;
                    }
                }
                1 => {
                    if self.cursor_y != 2 {
                        self.cursor_y += 1;
                    }
                }
                _ => panic!("Value must be -1 or 1"),
            }
        }
    }
    pub fn select_case(&mut self) {
        if self.board[self.cursor_y][self.cursor_x] == ' ' {
            self.board[self.cursor_y][self.cursor_x] = self.next_player;
            self.check_win();
        }
    }

    pub fn switch_next_player(&mut self) {
        match self.next_player {
            'X' => self.next_player = 'O',
            _ => self.next_player = 'X',
        }
    }

    pub fn ascii_current_case(&mut self, character: char) -> Vec<Line> {
        match character {
            'O' => {
                vec![
                    Line::from(Span::raw("   ____  ")),
                    Line::from(Span::raw("  / __ \\")),
                    Line::from(Span::raw(" | |  | |")),
                    Line::from(Span::raw(" | |  | |")),
                    Line::from(Span::raw(" | |__| |")),
                    Line::from(Span::raw("  \\____/ ")),
                ]
            }

            'X' => vec![
                Line::from(Span::raw("  __   __  ")),
                Line::from(Span::raw(" \\ \\ / /")),
                Line::from(Span::raw("  \\ V / ")),
                Line::from(Span::raw("   > <  ")),
                Line::from(Span::raw("  / . \\ ")),
                Line::from(Span::raw(" /_/ \\_\\")),
            ],
            _ => vec![
                Line::from(Span::raw("        ")),
                Line::from(Span::raw("        ")),
                Line::from(Span::raw("        ")),
                Line::from(Span::raw("        ")),
                Line::from(Span::raw("        ")),
                Line::from(Span::raw("        ")),
            ],
        }
    }

    pub fn check_win(&mut self) {
        self.case_left -= 1;
        // check row win
        for row in 0..3 {
            if self.board[row][0] != ' '
                && self.board[row][0] == self.board[row][1]
                && self.board[row][1] == self.board[row][2]
            {
                self.winner = self.board[row][0];
            }
        }

        // check col win
        for col in 0..3 {
            if self.board[0][col] != ' '
                && self.board[0][col] == self.board[1][col]
                && self.board[1][col] == self.board[2][col]
            {
                self.winner = self.board[0][col];
            }
        }

        // check for diagonal win
        if self.board[0][0] != ' '
            && self.board[0][0] == self.board[1][1]
            && self.board[0][0] == self.board[2][2]
        {
            self.winner = self.board[0][0];
        }
        // check for diagonal win other side
        if self.board[0][2] != ' '
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            self.winner = self.board[0][2];
        }
        // else we switch the player
        else {
            self.switch_next_player();
        }
    }

    pub fn is_game_end(&mut self) -> bool {
        if self.winner != ' ' || self.case_left == 0 {
            return true;
        }
        false
    }

    pub fn reload_game(&mut self) {
        if self.is_game_end() {
            *self = GameState::default();
        }
    }
}
