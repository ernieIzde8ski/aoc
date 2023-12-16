use common::{read_input, List};

fn hash_algo(ascii: &str) -> usize {
    ascii
        .bytes()
        .fold(0, |r, b| ((r + (b as usize)) * 17) % 256)
}

fn part_1(input: &str) -> usize {
    input
        .split(',')
        .map(|s| {
            s.chars()
                .filter(|c| !c.is_ascii_whitespace())
                .collect::<String>()
        })
        .map(|s| hash_algo(&s))
        .sum()
}

fn part_2(input: &str) -> usize {
    let seq = input
        .split(',')
        .map(|s| {
            let mut chars = s.chars().filter(|c| !c.is_ascii_whitespace());
            let mut left = String::new();
            for s in chars.by_ref() {
                match s {
                    '=' => break,
                    '-' => return (left, None),
                    c if c.is_alphabetic() => left.push(c),
                    c => panic!("unexpected char: '{c}'"),
                }
            }
            let right: u32 = chars.collect::<String>().parse().expect("should be an int");
            (left, Some(right))
        })
        .collect::<List<(String, Option<u32>)>>();

    let mut resp: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];

    for (key, focal_len) in seq.iter() {
        let slot: &mut Vec<(String, u32)> = {
            let i = hash_algo(key);
            &mut resp[i]
        };

        let existing_match = slot.iter().position(|(old_key, _)| old_key == key);

        match (existing_match, focal_len) {
            (None, None) => (),
            (Some(i), None) => {
                slot.remove(i);
            }
            (None, Some(focal_len)) => slot.push((key.clone(), *focal_len)),
            (Some(i), Some(focal_len)) => slot[i] = (key.clone(), *focal_len),
        }
    }

    resp.into_iter()
        .enumerate()
        .flat_map(|(i, arr)| {
            let i = i + 1;
            arr.into_iter()
                .enumerate()
                .map(move |(j, (_, flen))| i * (j + 1) * (flen as usize))
        })
        .sum()
}

// Sample output: 1320, 145
fn main() {
    let input = read_input!();
    let sum = part_1(&input);
    let focusing_power = part_2(&input);
    println!("{sum}, {focusing_power}")
}
