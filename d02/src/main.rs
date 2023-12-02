use anyhow::{anyhow, bail, Result};
use common::read_input;
use std::rc::Rc;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn value(&self) -> usize {
        match self {
            Self::Red => 0,
            Self::Green => 1,
            Self::Blue => 2,
        }
    }
}

/// Cubes are comma-separated
#[derive(Debug)]
struct Cube(Color, u32);
/// Rounds are semicolon-separated
type Round = Vec<Cube>;
/// Games are newline-separated
type Game = Vec<Round>;

fn parse_game(line: &str) -> Result<Game> {
    let mut resp = Game::new();

    let rounds = match line.split(": ").nth(1) {
        Some(s) => s,
        None => bail!("Missing games: '{line}'"),
    };

    for round_str in rounds.split("; ") {
        let mut round = Round::new();
        for cube in round_str.split(", ") {
            use Color::*;

            let mut cube = cube.split(' ');

            let num: u32 = cube
                .next()
                .ok_or_else(|| anyhow!("couldn't get the num val!"))?
                .parse()?;

            let color = match cube.next().and_then(|s| s.chars().next()) {
                Some('r') => Red,
                Some('g') => Green,
                Some('b') => Blue,
                c => bail!("unexpected value: {c:?}"),
            };

            round.push(Cube(color, num))
        }
        resp.push(round);
    }
    Ok(resp)
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Game>> + '_ {
    input.lines().map(parse_game)
}

/// Check if each round in a game is possible to be validly played.
fn possible_game(game: &Game) -> bool {
    game.iter().flatten().all(|cube| match cube {
        Cube(Color::Red, n) => *n <= 12,
        Cube(Color::Green, n) => *n <= 13,
        Cube(Color::Blue, n) => *n <= 14,
    })
}

/// Sum up the IDs of games with all valid rounds.
fn possible_games_id_sum(games: &[Game]) -> usize {
    let mut sum = 0;

    for (i, game) in games.iter().enumerate() {
        if possible_game(game) {
            // for some baffling reason, aoc problems are 1-indexed
            sum += i + 1;
        }
    }

    sum
}

/// Sum up the "power" of each round of games.
/// "Power" is the product of the minimum required cubes for a round to have been played.
fn min_possible_game_power_sum(games: &[Game]) -> u32 {
    let mut sum = 0;

    for game in games {
        let mut min_req = [0, 0, 0];

        for Cube(color, num) in game.iter().flatten() {
            let v = color.value();
            if min_req[v] < *num {
                min_req[v] = *num;
            }
        }

        sum += min_req[0] * min_req[1] * min_req[2];
    }

    sum
}

// Sample output: 8, 2286
fn main() -> Result<()> {
    let input = read_input!();
    let games = parse_input(&input).collect::<Result<Rc<[Game]>>>()?;
    let id_sum = possible_games_id_sum(&games);
    let prod_sum = min_possible_game_power_sum(&games);
    println!("{id_sum}, {prod_sum}");
    Ok(())
}
