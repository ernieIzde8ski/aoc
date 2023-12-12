use common::read_input;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse_rows(input: &str) -> Vec<(Vec<Spring>, Vec<u32>)> {
    fn parse_spring(c: char) -> Spring {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("unexpected char: {c}"),
        }
    }

    let mut resp = Vec::new();
    for line in input.lines() {
        let mut line = line.split_ascii_whitespace();

        let springs = line
            .next()
            .expect("should find springs")
            .chars()
            .map(parse_spring)
            .collect();

        let expected_columns = line
            .next()
            .expect("should find num8ers")
            .split(',')
            .map(|n| n.parse().expect("should be numeric"))
            .collect();

        resp.push((springs, expected_columns));
    }
    resp
}

fn evaluate_arrangement(springs: &[Spring]) -> Vec<u32> {
    let mut resp = Vec::new();
    let mut count = 0;
    for spring in springs {
        if *spring == Spring::Damaged {
            count += 1;
        } else if count > 0 {
            resp.push(count);
            count = 0;
        }
    }
    if count > 0 {
        resp.push(count);
    }
    resp
}

fn bruteforce_arrangements(springs: &[Spring], expected_columns: &[u32]) -> u32 {
    let mut springs: Vec<Spring> = springs.to_vec();

    let unknown_positions = springs
        .iter()
        .enumerate()
        .filter_map(|(i, spring)| match spring {
            Spring::Unknown => Some(i),
            _ => None,
        })
        .collect::<Vec<_>>();

    let unknown_combinations = (0..=unknown_positions.len())
        .flat_map(|r| unknown_positions.iter().cloned().combinations(r));

    let mut sum = 0;
    for damaged in unknown_combinations {
        let operational = unknown_positions
            .iter()
            .cloned()
            .filter(|i| !damaged.contains(i))
            .collect::<Vec<_>>();
        for i in damaged {
            springs[i] = Spring::Damaged;
        }
        for i in operational {
            springs[i] = Spring::Operational;
        }

        let evaluation = evaluate_arrangement(&springs);
        if expected_columns == evaluation {
            sum += 1;
        }
    }

    sum
}

fn sum_possible_arrangements(rows: &[(Vec<Spring>, Vec<u32>)]) -> u32 {
    rows.iter()
        .map(|(l, r)| bruteforce_arrangements(l, r))
        .sum()
}

/// Too lazy to figure out part 2, sorry. Looks neat though. Always
/// wondered how a computer would play Nonograms. The game is easily
/// played manually at any rate.
/// 
/// Sample output: 21
fn main() {
    let input = read_input!();
    let rows = parse_rows(&input);
    let resp = sum_possible_arrangements(&rows);
    println!("{resp}")
}
