use std::{fmt::Write, rc::Rc};

use common::read_input;

const BOUND_A: u8 = 'a' as u8;
const BOUND_Z: u8 = 'z' as u8;
const ASIZE: usize = (BOUND_Z - BOUND_A + 2) as usize;

/// Finds the most and least frequent characters in each column of the sample file.
/// Sample output should be:
///     most:  easter
///     least: advent
fn main() {
    let input = read_input!();
    let lines: Rc<[&[u8]]> = input.lines().map(|s| s.as_bytes()).collect();

    let mut most_frequent_chars = String::new();
    let mut least_frequent_chars = String::new();

    for col in 0..(lines[0].len()) {
        let mut arr = [0_u8; ASIZE];
        let mut biggest = (0_usize, 0_u8);
        for line in lines.iter() {
            let i = (line[col] ^ 96) as usize;
            arr[i] += 1;
            if arr[i] > biggest.1 {
                biggest = (i, arr[i]);
            }
        }

        let smallest = arr
            .into_iter()
            .enumerate()
            .filter(|s| s.1 != 0)
            .min_by_key(|s| s.1)
            .expect("should find the smallest char");

        most_frequent_chars
            .write_char((biggest.0 as u8 + 96) as char)
            .expect("writing to string buffer");

        least_frequent_chars
            .write_char((smallest.0 as u8 + 96) as char)
            .expect("writing to string buffer");
    }
    println!("most:  {most_frequent_chars}");
    println!("least: {least_frequent_chars}");
}
