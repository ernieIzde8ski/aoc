use common::read_input;

fn total_winning_options(race_duration: u64, highscore: u64) -> u64 {
    let mut n = 0;
    for held_for in 1..race_duration {
        let distance_traveled = held_for * (race_duration - held_for);
        if distance_traveled > highscore {
            n += 1;
        }
    }
    n
}

fn part_1(input: &str) -> u64 {
    let mut input = input.lines().map(|s| {
        s.split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().expect("should be a number"))
    });

    let times = input.next().expect("should exist");
    let distances = input.next().expect("should exist");
    debug_assert!(input.next().is_none());

    times
        .zip(distances)
        .map(|(a, b)| total_winning_options(a, b))
        .product()
}

fn part_2(input: &str) -> u64 {
    let mut input = input.lines().map(|s| {
        s.split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<u64>()
            .expect("should be an integer")
    });
    let race_duration = input.next().expect("should be an integer");
    let race_highscore = input.next().expect("should be an integer");
    total_winning_options(race_duration, race_highscore)
}

// Sample output: 288, 71503
fn main() {
    let input = read_input!();
    println!("{}, {}", part_1(&input), part_2(&input));
}
