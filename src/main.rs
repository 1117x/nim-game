mod nim;

use std::{
    fmt::Display,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveDown, MoveRight, MoveTo, MoveToNextLine},
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Color, Print, PrintStyledContent, Stylize},
    terminal::{self, ClearType, EnterAlternateScreen},
};
use nim::{Move, NimGame, Row};

enum PlayerType {
    Human(String),
    Computer,
}

impl PlayerType {
    fn get_move(&self, game: &NimGame) -> Option<Move> {
        match self {
            PlayerType::Human(_) => todo!(),
            PlayerType::Computer => Some(game.auto_move()),
        }
    }
}

impl Display for PlayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerType::Human(s) => f.write_str(s),
            PlayerType::Computer => f.write_str("AI"),
        }
    }
}

#[derive(Debug)]
struct GameDisplay {
    game: NimGame,
    width: u16,
    last_move: Option<Move>,
    highlighted_move: Move,
}

impl GameDisplay {
    fn display(&self) {
        let mut stdout = stdout();
        queue!(stdout, terminal::Clear(ClearType::All)).unwrap();
        queue!(stdout, MoveTo(0, 0)).unwrap();

        for (i, (&num_items, &num_initial_items)) in self
            .game
            .rows
            .iter()
            .zip(self.game.initial.iter())
            .enumerate()
        {
            let left_padding = self.width - num_initial_items as u16;

            let highlight_current_move = if i == self.highlighted_move.0 {
                self.highlighted_move.1
            } else {
                0
            };

            let unmoved_row_string: String =
                "◉ ".repeat((num_items - highlight_current_move).into());

            let moved_row_string: String = "◉ ".repeat(highlight_current_move.into());

            let highlight_last_move = if let Some((row, count)) = self.last_move {
                if i == row {
                    count
                } else {
                    0
                }
            } else {
                0
            };

            let highlight_last_move_string = "◎ ".repeat(highlight_last_move.into());

            let empty_string: String =
                "◎ ".repeat((num_initial_items - num_items - highlight_last_move).into());
            queue!(
                stdout,
                MoveRight(left_padding),
                Print(unmoved_row_string),
                PrintStyledContent(moved_row_string.with(Color::Red)),
                PrintStyledContent(highlight_last_move_string.with(Color::Blue)),
                Print(empty_string),
                MoveToNextLine(1)
            )
            .unwrap();

            stdout.flush().unwrap();
        }
    }

    fn update_loop(mut self) {
        self.display();
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                let changed = match event::read().unwrap() {
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Esc,
                        modifiers: KeyModifiers::NONE,
                    }) => break,
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Right,
                        modifiers: KeyModifiers::NONE,
                    }) if self.highlighted_move.1 > 1 => {
                        self.highlighted_move.1 -= 1;
                        true
                    }
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Left,
                        modifiers: KeyModifiers::NONE,
                    }) if self.highlighted_move.1 < self.game.rows[self.highlighted_move.0] => {
                        self.highlighted_move.1 += 1;
                        true
                    }
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Down,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        self.highlighted_move.0 =
                            (self.highlighted_move.0 + 1) % self.game.rows.len();
                        self.highlighted_move.1 = 1;
                        true
                    }
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Up,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        if self.highlighted_move.0 == 0 {
                            self.highlighted_move.0 = (self.game.rows.len() - 1).into();
                        } else {
                            self.highlighted_move.0 -= 1;
                        }
                        self.highlighted_move.1 = 1;
                        true
                    }
                    event::Event::Key(KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                    }) => {
                        self.game.apply_move(self.highlighted_move);
                        self.last_move = Some(self.highlighted_move);
                        self.highlighted_move = self
                            .game
                            .rows
                            .iter()
                            .enumerate()
                            .find_map(|(i, &x)| if x > 0 { Some((i,1)) } else { None })
                            .unwrap();
                        true
                    }
                    _ => false,
                };
                if changed {
                    self.display();
                }
            }
        }
    }

    fn new() -> Self {
        let g = NimGame::new(vec![1, 3, 5, 7]);

        let &largest_row = g.initial.iter().max().unwrap();

        GameDisplay {
            game: g,
            width: largest_row.into(),
            last_move: None,
            highlighted_move: (0, 1),
        }
    }
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

    GameDisplay::new().update_loop();
}
