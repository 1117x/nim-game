use std::fmt::Display;

use rand::prelude::SliceRandom;

pub type Row = u8;
pub type Move = (usize, Row);

#[derive(Debug)]
pub struct NimGame {
    pub rows: Vec<Row>,
    pub initial: Vec<Row>,
}

impl NimGame {
    pub fn new(initial: Vec<Row>) -> Self {
        NimGame {
            rows: initial.clone(),
            initial,
        }
    }

    /// apply a move to the current game
    /// returns whether the move was valid
    /// If the move is not valid, the game state does not change
    pub fn apply_move(&mut self, m: Move) -> bool {
        if self.rows.get(m.0).filter(|c| **c >= m.1).is_some() && m.1 > 0 {
            self.rows[m.0] -= m.1;
            true
        } else {
            false
        }
    }

    /// Special cases for the end game
    fn winning_move(&self) -> Option<Move> {
        let mut v: Vec<_> = self.rows.iter().enumerate().collect();
        v.sort_by_key(|x| -(*x.1 as i8));

        match &v[..4] {
            [(i, x), (_, 1), (_, 1), (_, 0)] if **x > 1 => Some((*i, (*x - 1))),
            [(i, x), (_, 2), (_, 0)] if **x > 2 => Some((*i, (*x - 2))),
            [(i, x), (_, 1), (_, 0), ..] => Some((*i, **x)),
            [(i, x), (_, 0), ..] => Some((*i, (*x - 1))),
            _ => None,
        }
    }

    /// Try to find a move that can force a win
    fn xor_move(&self) -> Option<Move> {
        let column_parity = self.rows.iter().fold(0, |x, y| x ^ y);
        if column_parity == 0 {
            return None;
        }

        let m = self
            .rows
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                let take = row ^ column_parity;
                if take < *row {
                    Some((i, row - take))
                } else {
                    None
                }
            })
            .unwrap();

        Some(m)
    }

    /// Remove 1 from a random row
    fn random_move(&self) -> Move {
        let possible: Vec<_> = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x > 0 { Some(i) } else { None })
            .collect();

        let row = possible.choose(&mut rand::thread_rng()).unwrap();

        (*row, 1)
    }

    /// calculate AI move
    pub fn auto_move(&self) -> Move {
        // first try if we can win in the end game
        self.winning_move()
            // then try to force a win
            .or_else(|| self.xor_move())
            // otherwise, make a random move
            .unwrap_or_else(|| self.random_move())
    }

    /// Check whether the game is lost
    pub fn check_lose(&self) -> bool {
        !self.rows.iter().any(|x| *x > 1) && self.rows.iter().filter(|x| **x == 1).count() == 1
    }
}

impl Display for NimGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_width = self.initial.iter().max().unwrap();

        for (i, (row, initial)) in self.rows.iter().zip(self.initial.iter()).enumerate() {
            let bars = "| ".repeat(*row as usize);

            let dots = ". ".repeat((initial - row) as usize);

            let padding = " ".repeat((max_width - initial) as usize);

            write!(f, "{} {}{}{}\n", i, padding, bars, dots)?;
        }
        write!(f, "---------------")
    }
}