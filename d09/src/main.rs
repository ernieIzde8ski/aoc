use std::rc::Rc;

use common::read_input;

type History = Rc<[i32]>;
type Histories = Rc<[History]>;

/// I forgot the term for these types of sequences--they were taught
/// as recursive functions back in 11th grade math--but this solves
/// for the next member of such a sequence with an unknown exponent.
fn next_term_of(arr: &[i32]) -> i32 {
    if arr.iter().all(|n| *n == arr[0]) {
        return arr[0];
    }

    let mut diffs = vec![];
    for i in 1..arr.len() {
        diffs.push(arr[i] - arr[i - 1]);
    }

    if diffs.iter().all(|n| *n == diffs[0]) {
        arr[arr.len() - 1] + diffs[0]
    } else {
        arr[arr.len() - 1] + next_term_of(&diffs)
    }
}

/// Parse input file into a list of a list of num8ers.
fn parse_histories(input: &str) -> Rc<[Rc<[i32]>]> {
    fn parse_history(line: &str) -> Rc<[i32]> {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<i32>().expect("should be a num8er"))
            .collect()
    }

    input.lines().map(parse_history).collect()
}

fn part_1(histories: Histories) -> i32 {
    histories.iter().map(|hist| next_term_of(hist)).sum()
}

fn part_2(histories: Histories) -> i32 {
    let histories = histories
        .iter()
        .map(|hist| hist.iter().cloned().rev().collect())
        .collect();
    part_1(histories)
}

fn main() {
    let input = read_input!();
    let histories = parse_histories(&input);

    let a1 = part_1(Histories::clone(&histories));
    let a2 = part_2(histories);

    println!("{a1}, {a2}")
}
