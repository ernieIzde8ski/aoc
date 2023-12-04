use std::rc::Rc;

use common::read_input;

#[derive(Debug)]
struct Card {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Card {
    fn matches(&self) -> u32 {
        let mut sum = 0;
        for i in &self.right {
            if self.left.contains(i) {
                sum += 1;
            }
        }
        sum
    }
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(|line| {
        let mut line = line.split_ascii_whitespace().skip(2);

        let mut left = vec![];
        for slice in line.by_ref() {
            if slice == "|" {
                break;
            }
            let num: u32 = slice.parse().expect("should contain a number");
            left.push(num);
        }

        let mut right = vec![];
        for slice in line {
            let num: u32 = slice.parse().expect("should contain a number");
            right.push(num);
        }
        Card { left, right }
    })
}

/// Gets the value of a deck of cards by their values
fn worth_by_value(cards: &[Card]) -> u32 {
    cards.iter().map(|card| 2_u32.pow(card.matches()) / 2).sum()
}

/// Figures out how many "copies" of each card there should be, and
/// returns the total number of those copies
fn worth_by_copies(cards: &[Card]) -> u32 {
    let mut copies = vec![1; cards.len()];
    #[allow(clippy::needless_range_loop)]
    for (i, card) in cards.iter().enumerate() {
        let c = copies[i];
        let matches = card.matches() as usize;
        for j in (i + 1)..(i + matches + 1) {
            copies[j] += c;
        }
    }
    copies.iter().sum::<usize>() as u32
}

/// Sample output: 13, 30
fn main() {
    let input = read_input!();
    let cards: Rc<[Card]> = parse_cards(&input).collect();
    let p1 = worth_by_value(&cards);
    let p2 = worth_by_copies(&cards);
    println!("{p1}, {p2}");
}
