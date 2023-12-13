use core::panic;

use common::{read_input, List};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Sample {
    Ash,
    Rock,
}

fn parse_patterns(input: &str) -> List<List<List<Sample>>> {
    fn parse_pattern<'a>(input: impl Iterator<Item = &'a str>) -> List<List<Sample>> {
        input
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Sample::Rock,
                        '.' => Sample::Ash,
                        c => panic!("unexpected char: {c}"),
                    })
                    .collect::<List<_>>()
            })
            .collect::<List<_>>()
    }
    input
        .lines()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(b, p)| match b {
            true => Some(p),
            false => None,
        })
        .map(parse_pattern)
        .collect::<List<_>>()
}

fn part_1(pattern: List<List<Sample>>) -> usize {
    let column = |idx: usize| {
        pattern
            .clone()
            .iter()
            .map(|row| row[idx])
            .collect::<List<_>>()
    };
    'check_columns: for i in 1..pattern[0].len() {
        let (mut j, mut k) = (i, i - 1);
        while j > 0 && k < (pattern[0].len() - 1) {
            j -= 1;
            k += 1;

            if column(j) != column(k) {
                continue 'check_columns;
            }
        }
        return i;
    }

    'check_rows: for i in 1..pattern.len() {
        let (mut j, mut k) = (i, i - 1);
        while j > 0 && k < pattern.len() - 1 {
            j -= 1;
            k += 1;
            if pattern[j] != pattern[k] {
                continue 'check_rows;
            }
        }
        return i * 100;
    }

    panic!("Couldn't find a reflection!");
}

fn different_values<T: PartialEq>(left: &[T], right: &[T]) -> usize {
    left.iter().zip(right).filter(|(l, r)| l != r).count()
}

fn part_2(pattern: List<List<Sample>>) -> usize {
    let column = |idx: usize| {
        pattern
            .clone()
            .iter()
            .map(|row| row[idx])
            .collect::<List<_>>()
    };

    'check_columns: for i in 1..pattern[0].len() {
        let mut fault_tolerated = false;
        let (mut j, mut k) = (i, i - 1);
        while j > 0 && k < (pattern[0].len() - 1) {
            j -= 1;
            k += 1;

            let j = column(j);
            let k = column(k);
            if j != k {
                if !fault_tolerated && different_values(&j, &k) == 1 {
                    // this might be just a smudge!
                    // we can tolerate 1 of those
                    fault_tolerated = true;
                } else {
                    continue 'check_columns;
                }
            }
        }

        // the smudge MUST be corrected.
        if !fault_tolerated {
            continue;
        }

        return i;
    }

    'check_rows: for i in 1..pattern.len() {
        let mut fault_tolerated = false;
        let (mut j, mut k) = (i, i - 1);
        while j > 0 && k < pattern.len() - 1 {
            j -= 1;
            k += 1;

            let j = &pattern[j];
            let k = &pattern[k];

            if j == k {
                continue;
            } else if !fault_tolerated && different_values(j, k) == 1 {
                fault_tolerated = true;
                continue;
            } else {
                continue 'check_rows;
            }
        }
        if !fault_tolerated {
            continue;
        }
        return i * 100;
    }

    panic!("Couldn't find a reflection!")
}

fn sum_reflections(
    patterns: List<List<List<Sample>>>,
    reflector: impl Fn(List<List<Sample>>) -> usize,
) -> usize {
    patterns.iter().cloned().map(reflector).sum()
}

fn main() {
    let input = read_input!();
    let patterns = parse_patterns(&input);

    let part_1 = sum_reflections(patterns.clone(), part_1);
    let part_2 = sum_reflections(patterns.clone(), part_2);

    println!("{part_1}, {part_2}");
}
