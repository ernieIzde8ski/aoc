mod initial_guide;
mod revised_guide;
use std::mem::discriminant;

use initial_guide::{InitialStrategyGuide, Matchup};
use revised_guide::RevisedStrategyGuide;

#[derive(Debug, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum MatchupResult {
    Win,
    Loss,
    Draw,
}

impl MatchupResult {
    fn value(&self) -> u32 {
        match self {
            MatchupResult::Win => 6,
            MatchupResult::Loss => 0,
            MatchupResult::Draw => 3,
        }
    }
}

impl Rps {
    fn value(&self) -> u32 {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    /// Returns the move which would beat this one.
    fn winner(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    /// Returns the move which would lose to this one.
    fn loser(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    /// Checks who would win between these players
    fn versus(&self, opponent: &Rps) -> MatchupResult {
        if discriminant(self) == discriminant(&opponent.winner()) {
            MatchupResult::Win
        } else if discriminant(&self.winner()) == discriminant(opponent) {
            MatchupResult::Loss
        } else {
            MatchupResult::Draw
        }
    }
}

fn main() {
    let input = common::read_input!();

    let initial_score = InitialStrategyGuide::from(input.as_ref()).predicted_score();
    let revised_score = RevisedStrategyGuide::from(input.as_ref()).predicted_score();

    println!("Predicted score from initial guide: {initial_score}");
    println!("Preditect score from revised guide: {revised_score}");
}
