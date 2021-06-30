mod nim;

use std::{fmt::Display, process::exit};

use clap::{App, Arg};
use nim::{Move, NimGame, Row};

enum PlayerType {
    Human(String),
    Computer,
}

impl PlayerType {
    fn get_move(&self, game: &NimGame) -> Option<Move> {
        match self {
            PlayerType::Human(_) => get_human_move(),
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

fn get_human_move() -> Option<Move> {
    println!("Which (Row,Count)?");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let split = line.trim().split(",").collect::<Vec<_>>();

    if split.len() != 2 {
        return None;
    }

    Some((split[0].parse().ok()?, split[1].parse().ok()?))
}

fn get_move(player: &PlayerType, g: &mut NimGame) {
    println!("{}", g);

    println!("It's {}'s turn!", player);

    let m = loop {
        if let Some(m) = player.get_move(g) {
            if g.apply_move(m) {
                break m;
            }
        }
        println!("Invalid move");
    };

    println!("{} takes {} from row {}", player, m.1, m.0);
}

fn parse_args() -> ([PlayerType; 2], Vec<Row>) {
    let matches = App::new("Nim")
        .arg(Arg::new("player1").index(1).default_value("human"))
        .arg(Arg::new("player2").index(2).default_value("ai"))
        .arg(Arg::new("rows").long("rows").default_value("1,3,5,7"))
        .get_matches();

    let initial_vec = match matches
        .value_of("rows")
        .unwrap()
        .split(",")
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(m) => m,
        Err(_) => {
            println!(
                "Error parsing row list \"{}\"",
                matches.value_of("rows").unwrap()
            );
            exit(1);
        }
    };

    fn parse_player(player: &str) -> PlayerType {
        match player {
            "AI" | "ai" => PlayerType::Computer,
            name => PlayerType::Human(name.to_owned()),
        }
    }

    let players = [
        parse_player(matches.value_of("player1").unwrap()),
        parse_player(matches.value_of("player2").unwrap()),
    ];

    (players, initial_vec)
}

fn main() {
    let (players, initial_vec) = parse_args();

    let mut g = NimGame::new(initial_vec);

    let winner = loop {
        get_move(&players[0], &mut g);

        if g.check_lose() {
            break &players[0];
        }

        get_move(&players[1], &mut g);

        if g.check_lose() {
            break &players[1];
        }
    };

    println!("{}", g);
    println!("{} has won!", winner);
}
