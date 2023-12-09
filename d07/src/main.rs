use std::{cmp::Ordering, collections::HashMap};

use anyhow::{bail, Result};
use common::read_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOf,
    FourOf,
    FullHouse,
    ThreeOf,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

impl Hand {
    fn kind(&self) -> HandKind {
        debug_assert!(self.cards.len() == 5);

        let mut set: HashMap<char, u32> = HashMap::new();
        for card in self.cards.chars() {
            match set.get_mut(&card) {
                Some(v) => *v += 1,
                None => {
                    let _ = set.insert(card, 1);
                }
            }
        }

        let mut set = set.into_values().collect::<Vec<u32>>();
        set.sort();
        use HandKind::*;
        match &set[..] {
            [5] => FiveOf,
            [1, 4] => FourOf,
            [2, 3] => FullHouse,
            [1, 1, 3] => ThreeOf,
            [1, 2, 2] => TwoPair,
            [1, 1, 1, 2] => OnePair,
            [1, 1, 1, 1, 1] => HighCard,
            _ => panic!("Unexpected card: {}", self.cards),
        }
    }
    fn card_value(card: char) -> Result<i8> {
        if let Some(num) = card.to_digit(10) {
            return Ok(12 - (num as i8 - 2));
        };
        let n = match card {
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            c => bail!("unexpected char: {c}"),
        };
        Ok(12 - n)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind().cmp(&other.kind()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        };

        let matching = self.cards.chars().zip(other.cards.chars());
        for (s_card, o_card) in matching {
            let s_value = Self::card_value(s_card).unwrap();
            let o_value = Self::card_value(o_card).unwrap();
            match s_value.cmp(&o_value) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }

        Ordering::Equal
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let mut resp = Vec::new();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();
        let cards = line
            .next()
            .expect("should be a string of values")
            .to_owned();
        let bid = line
            .next()
            .expect("should be a num")
            .parse()
            .expect("should be a num");
        let hand = Hand { cards, bid };
        resp.push(hand);
    }
    resp
}

/* I accidentally deleted the code that does part 2, sorry.
It shouldn't be hard to recreate, anyways. Change the value of 'J'
to -1 in Hand::card_value and implement joker card logic in Hand::kind.

Sample output: 6440 */
fn main() {
    let input = read_input!();
    let mut hands = parse_hands(&input);
    hands.sort();
    hands.reverse();
    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();
    println!("{sum}");
}
