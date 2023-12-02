use std::rc::Rc;

use common::read_input;

fn part_1(input: &str) -> (Option<u32>, Option<u32>) {
    let mut nums = input.chars().filter(|c| c.is_numeric());
    let sub_48_as_u32 = |n| Some((n as u32) - 48);
    let first: Option<u32> = nums.next().and_then(sub_48_as_u32);
    let last: Option<u32> = nums.last().and_then(sub_48_as_u32);
    (first, last)
}

fn part_2(line: &str) -> (Option<u32>, Option<u32>) {
    let mut i = 0;
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    let chars: Rc<[char]> = line.chars().collect();

    while i < chars.len() {
        macro_rules! case {
            ($res:expr) => {{
                let next = Some($res);
                match first {
                    None => first = next,
                    Some(_) => last = next,
                };
                i += 1;
            }};
        }

        match chars[i..] {
            ['0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9', ..] => {
                match first {
                    None => first = chars[i].to_digit(10),
                    Some(_) => last = chars[i].to_digit(10),
                };
                i += 1;
            }
            ['z', 'e', 'r', 'o', ..] => case!(0),
            ['o', 'n', 'e', ..] => case!(1),
            ['t', 'w', 'o', ..] => case!(2),
            ['t', 'h', 'r', 'e', 'e', ..] => case!(3),
            ['f', 'o', 'u', 'r', ..] => case!(4),
            ['f', 'i', 'v', 'e', ..] => case!(5),
            ['s', 'i', 'x', ..] => case!(6),
            ['s', 'e', 'v', 'e', 'n', ..] => case!(7),
            ['e', 'i', 'g', 'h', 't', ..] => case!(8),
            ['n', 'i', 'n', 'e', ..] => case!(9),
            [..] => i += 1,
        }
    }
    (first, last)
}

/// Takes a function that returns the first and last node,
fn apply_input_parser(input: &str, parser: impl Fn(&str) -> (Option<u32>, Option<u32>)) -> u32 {
    let mut resp = 0;
    for line in input.lines() {
        let (first, last) = parser(line);
        let first = first.unwrap();
        let last = last.unwrap_or(first);
        resp += first * 10 + last;
    }
    resp
}

// Sample output: 497, 442
fn main() {
    let input = read_input!();
    println!(
        "{}, {}",
        apply_input_parser(&input, part_1),
        apply_input_parser(&input, part_2),
    );
}
