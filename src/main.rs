mod nim;

use std::{
    fmt::Display,
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveDown, MoveRight, MoveTo, MoveToNextLine},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Color, Print, PrintStyledContent, Stylize},
    terminal::{self, ClearType, EnterAlternateScreen},
};
use nim::{Move, NimGame, Row};

enum PlayerType {
    Human(String),
    Computer,
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
    width: u16,
    last_move: Option<Move>,
    highlighted_move: Move,
}

impl GameDisplay {
    fn display(&self, game: &NimGame, highlighted: bool) {
        let mut stdout = stdout();
        queue!(stdout, terminal::Clear(ClearType::All)).unwrap();
        queue!(stdout, MoveTo(0, 0)).unwrap();

        for (i, (&num_items, &num_initial_items)) in
            game.rows.iter().zip(game.initial.iter()).enumerate()
        {
            let left_padding = self.width - num_initial_items as u16;

            let highlight_current_move = if i == self.highlighted_move.0 && highlighted {
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

    fn get_user_move(&mut self, game: &NimGame) -> Option<Move> {
        self.highlighted_move.0 = game
            .rows
            .iter()
            .enumerate()
            .find_map(|(i, &x)| if x > 0 { Some(i) } else { None })
            .unwrap();
        self.highlighted_move.1 = 1;

        self.display(game, true);
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(KeyEvent { code: key, .. }) = event::read().unwrap() {
                    if match key {
                        KeyCode::Esc => return None,
                        KeyCode::Left => {
                            self.move_selection_left(game);
                            true
                        }
                        KeyCode::Right => {
                            self.move_selection_right();
                            true
                        }
                        KeyCode::Up => {
                            self.move_selection_up(game);
                            true
                        }
                        KeyCode::Down => {
                            self.move_selection_down(game);
                            true
                        }
                        KeyCode::Enter => return Some(self.highlighted_move),
                        _ => false,
                    } {
                        self.display(game, true);
                    }
                };
            }
        }
    }

    fn move_selection_up(&mut self, game: &NimGame) {
        loop {
            if self.highlighted_move.0 == 0 {
                self.highlighted_move.0 = game.rows.len() - 1;
            } else {
                self.highlighted_move.0 -= 1;
            }

            self.highlighted_move.1 = 1;

            if game.rows[self.highlighted_move.0] != 0 {
                break;
            }
        }
    }

    fn move_selection_down(&mut self, game: &NimGame) {
        loop {
            if self.highlighted_move.0 == game.rows.len() - 1 {
                self.highlighted_move.0 = 0;
            } else {
                self.highlighted_move.0 += 1;
            }

            self.highlighted_move.1 = 1;

            if game.rows[self.highlighted_move.0] != 0 {
                break;
            }
        }
    }

    fn move_selection_right(&mut self) {
        if self.highlighted_move.1 > 1 {
            self.highlighted_move.1 -= 1;
        }
    }

    fn move_selection_left(&mut self, game: &NimGame) {
        if self.highlighted_move.1 < game.rows[self.highlighted_move.0] {
            self.highlighted_move.1 += 1;
        }
    }

    fn new(g: &NimGame) -> GameDisplay {
        let &largest_row = g.initial.iter().max().unwrap();

        GameDisplay {
            width: largest_row.into(),
            last_move: None,
            highlighted_move: (0, 1),
        }
    }
}

impl PlayerType {
    fn get_move(&self, display: &mut GameDisplay, game: &NimGame) -> Option<Move> {
        match self {
            PlayerType::Human(_) => display.get_user_move(game),
            PlayerType::Computer => Some(game.auto_move()),
        }
    }

    fn apply_move(&self, display: &mut GameDisplay, game: &mut NimGame) -> Option<bool> {
        if let Some(m) = self.get_move(display, game) {
            game.apply_move(m);
            display.last_move = Some(m);
            Some(game.check_lose())
        } else {
            None
        }
    }
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();

    let mut g = NimGame::new(vec![1, 3, 5, 7]);
    let mut display = GameDisplay::new(&g);

    let players = [PlayerType::Computer, PlayerType::Human("Human".into())];

    let winner = loop {
        match players[0].apply_move(&mut display, &mut g) {
            Some(true) => break Some(&players[0]),
            None => break None,
            _ => {}
        }
        match players[1].apply_move(&mut display, &mut g) {
            Some(true) => break Some(&players[1]),
            None => break None,
            _ => {}
        }
    };

    display.display(&g, false);

    if let Some(winner) = winner {
        println!("{} has won!", winner);
    } else {
        println!("Aborting.");
    }
}
