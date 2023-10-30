use std::mem;

use common::read_input;

/// An array of elves, each carrying some total calories of food.
type Elves = Vec<u32>;

/// Parses input.txt into Elves.
fn parse_input(input: String) -> Elves {
    let mut resp = Vec::new();
    let mut current_elf = 0;
    // For this problem, each Elf is separated by two newlines,
    // and each new calorie is separated by one.
    // eg:  1000
    //
    //      2000
    //      3000
    //
    //      4000
    for line in input.lines() {
        if line.is_empty() {
            resp.push(mem::take(&mut current_elf));
        } else {
            current_elf += line.parse::<u32>().expect("should be an unsigned integer")
        }
    }

    if current_elf != 0 {
        resp.push(current_elf);
    }

    resp
}

fn main() {
    let mut elves = parse_input(read_input!());
    elves.sort();
    let len = elves.len();
    elves.sort();
    println!("The most caloric elf has {} total calories", elves[len - 1]);
    println!(
        "Most caloric elves have {:?} total calories",
        elves[(len - 4)..(len - 1)].iter().sum::<u32>()
    );
}
