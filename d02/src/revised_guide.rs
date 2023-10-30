use std::fmt::Debug;

use super::{Matchup, MatchupResult, RPS};

#[derive(Debug)]
struct UncalculatedMatchup {
    opponent_move: RPS,
    expected_condition: MatchupResult,
}

/// A matchup containing your opponents move and what you are being told to play.
impl UncalculatedMatchup {
    pub fn new(opponent_move: RPS, expected_condition: MatchupResult) -> Self {
        Self {
            opponent_move,
            expected_condition,
        }
    }
}

impl From<&UncalculatedMatchup> for Matchup {
    fn from(value: &UncalculatedMatchup) -> Self {
        let resp_move = match value.expected_condition {
            MatchupResult::Win => value.opponent_move.winner(),
            MatchupResult::Draw => value.opponent_move,
            MatchupResult::Loss => value.opponent_move.loser(),
        };
        Self::new(value.opponent_move, resp_move)
    }
}

pub struct RevisedStrategyGuide(Vec<UncalculatedMatchup>);

impl From<&str> for RevisedStrategyGuide {
    fn from(input: &str) -> Self {
        // initially did a Vec::with_capacity() by counting newlines,
        // but this turned out to be about twice as slow.
        // even Vec::with_capacity(2500) causes no noticeable performance gain
        let mut resp = Vec::new();

        let mut current_line = 0;
        let mut input = input.chars();

        while let Some(ch) = input.next() {
            use MatchupResult::*;
            use RPS::*;
            let opponent = match ch {
                'A' => Rock,
                'B' => Paper,
                'C' => Scissors,
                '\n' => {
                    current_line += 1;
                    continue;
                }
                _ => panic!("line {current_line}: expected [ABC]; got '{ch}'"),
            };

            match input.next() {
                Some(' ') => (),
                Some(c) => panic!("line {current_line}: expected a space; got '{c}'"),
                None => panic!("line {current_line}: expected a space; got EOF"),
            }

            let condition = match input.next() {
                Some('X') => Loss,
                Some('Y') => Draw,
                Some('Z') => Win,
                Some(c) => panic!("line {current_line}: expected a [XYZ]; got '{c}'"),
                None => panic!("line {current_line}: expected a space; got EOF"),
            };

            resp.push(UncalculatedMatchup::new(opponent, condition));
        }

        Self(resp)
    }
}

impl Debug for RevisedStrategyGuide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl RevisedStrategyGuide {
    pub fn predicted_score(&self) -> u32 {
        self.0.iter().map(|u| Matchup::from(u).score()).sum()
    }
}
