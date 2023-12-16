use std::collections::HashMap;

use anyhow::{bail, Error, Result};
use common::{read_input, List};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    LeftMirror,
    RightMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Tile::*;
        let resp = match c {
            '.' => Empty,
            '\\' => LeftMirror,
            '/' => RightMirror,
            '-' => HorizontalSplitter,
            '|' => VerticalSplitter,
            _ => bail!("unexpected char: '{c}'"),
        };
        Ok(resp)
    }
}

fn parse_tiles(input: &str) -> List<List<Tile>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(Tile::try_from)
                .collect::<Result<_>>()
                .expect("should be a list of tile")
        })
        .collect()
}

fn count_energized_tiles(
    tiles: List<List<Tile>>,
    starting_coord: (isize, isize),
    starting_direction: (isize, isize),
) -> usize {
    let mut pos: (isize, isize) = starting_coord;
    let mut dir: (isize, isize) = starting_direction;

    let mut positions: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    let mut inevaluated = vec![];

    'ploop: loop {
        macro_rules! split_case {
            ($a:expr, $b:expr) => {{
                inevaluated.push((pos, $a));
                $b
            }};
        }

        dir = match (tiles[pos.0 as usize][pos.1 as usize], dir) {
            (Tile::LeftMirror, (a, b)) => (b, a),
            (Tile::RightMirror, (a, 0)) => (0, -a),
            (Tile::RightMirror, (0, a)) => (-a, 0),
            (Tile::HorizontalSplitter, (0, a)) => (0, a),
            (Tile::HorizontalSplitter, (-1 | 1, 0)) => split_case!((0, -1), (0, 1)),
            (Tile::VerticalSplitter, (a, 0)) => (a, 0),
            (Tile::VerticalSplitter, (0, -1 | 1)) => split_case!((-1, 0), (1, 0)),
            (Tile::Empty, dir) => dir,
            (tile, dir) => panic!("Unexpected: {tile:?}, {dir:?}"),
        };

        match positions.get_mut(&pos) {
            None => {
                positions.insert(pos, vec![dir]);
            }
            Some(mut arr) => {
                while arr.contains(&dir) {
                    if inevaluated.is_empty() {
                        break 'ploop;
                    }

                    (pos, dir) = inevaluated.pop().expect("shouldn't be empty");
                    arr = match positions.get_mut(&pos) {
                        Some(a) => a,
                        None => {
                            positions.insert(pos, vec![]);
                            positions.get_mut(&pos).unwrap()
                        }
                    }
                }
                arr.push(dir);
            }
        };

        pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !(pos.0 >= 0
            && pos.0 < tiles.len() as isize
            && pos.1 >= 0
            && pos.1 < tiles[pos.0 as usize].len() as isize)
        {
            match inevaluated.pop() {
                Some((p, d)) => {
                    pos = p;
                    dir = d;
                }
                None => break,
            }
        }
    }
    positions.len()
}

fn part_1(tiles: List<List<Tile>>) -> usize {
    count_energized_tiles(tiles, (0, 0), (0, 1))
}

fn part_2(tiles: List<List<Tile>>) -> usize {
    let rows = tiles.len() as isize;
    let cols = tiles[0].len() as isize;

    let bottoms = (0..cols).map(|n| ((rows - 1, n), (-1, 0)));
    let tops = (0..cols).map(|n| ((0, n), (1, 0)));
    let lefts = (0..rows).map(|n| ((n, 0), (0, 1)));
    let rights = (0..rows).map(|n| ((n, cols - 1), (0, -1)));

    let coords = bottoms.chain(tops).chain(lefts).chain(rights);
    let mut biggest = 0;
    for (coord, dir) in coords {
        let size = count_energized_tiles(tiles.clone(), coord, dir);
        if size > biggest {
            biggest = size;
        }
    }
    biggest
}

// Sample output: 6883, 7228
fn main() {
    let input = read_input!();
    let tiles = parse_tiles(&input);
    println!("{}, {}", part_1(tiles.clone()), part_2(tiles));
}
