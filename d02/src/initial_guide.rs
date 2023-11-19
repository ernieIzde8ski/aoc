use super::Rps;

#[derive(Debug)]
pub(crate) struct Matchup {
    /// The move your opponent is expected to play.
    opponent: Rps,
    /// The move suggested by the strategy guide.
    response: Rps,
}

impl Matchup {
    pub fn new(opponent: Rps, response: Rps) -> Self {
        Self { opponent, response }
    }

    /// Calculates score by adding up the value of our move and the value of the matchup.
    pub fn score(&self) -> u32 {
        self.response.value() + self.response.versus(&self.opponent).value()
    }
}

pub struct InitialStrategyGuide(Vec<Matchup>);

impl From<&str> for InitialStrategyGuide {
    fn from(input: &str) -> Self {
        // initially did a Vec::with_capacity() by counting newlines,
        // but this turned out to be about twice as slow
        let mut resp = Vec::new();

        let mut current_line = 0;
        let mut input = input.chars();

        while let Some(ch) = input.next() {
            use Rps::*;
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

            let response = match input.next() {
                Some('X') => Rock,
                Some('Y') => Paper,
                Some('Z') => Scissors,
                Some(c) => panic!("line {current_line}: expected a [XYZ]; got '{c}'"),
                None => panic!("line {current_line}: expected a space; got EOF"),
            };

            resp.push(Matchup::new(opponent, response));
        }

        InitialStrategyGuide(resp)
    }
}

impl InitialStrategyGuide {
    pub fn predicted_score(&self) -> u32 {
        self.0.iter().map(|m| m.score()).sum()
    }
}
