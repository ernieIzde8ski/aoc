use std::{mem, num::ParseIntError, rc::Rc};

use anyhow::{bail, Result};
use common::read_input;

fn main() -> Result<()> {
    let input = read_input!();
    let mut input = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(str::split_ascii_whitespace);

    let mut seeds = input
        .next()
        .unwrap()
        .skip(1)
        .map(str::parse)
        .collect::<Result<Vec<i64>, ParseIntError>>()
        .unwrap();

    // seeds = (0..100).collect();

    let mut maps = vec![];
    let mut map = vec![];
    for line in input {
        let line: Rc<[&str]> = line.collect();
        match line.len() {
            2 => {
                if !map.is_empty() {
                    maps.push(mem::take(&mut map));
                }
            }
            3 => {
                let func = str::parse::<i64>;
                let mapping = (func(line[0])?, func(line[1])?, func(line[2])?);
                map.push(mapping)
            }
            _ => bail!("invalid line: {line:?}"),
        }
    }

    if !map.is_empty() {
        maps.push(mem::take(&mut map));
    }

    for map in maps {
        let mut diffs = vec![None; seeds.len()];

        for (dest_start, source_start, len) in map {
            let diff = dest_start - source_start;
            let range = source_start..(source_start + len);
            for (i, seed) in seeds.iter().enumerate() {
                if diffs[i].is_none() && range.contains(seed) {
                    diffs[i] = Some(diff);
                }
            }
        }

        let joined = seeds.iter_mut().zip(diffs);
        for (seed, diff) in joined {
            if let Some(diff) = diff {
                *seed += diff;
            }
        }
    }

    println!("{}", seeds.iter().min().unwrap());
    Ok(())
}
