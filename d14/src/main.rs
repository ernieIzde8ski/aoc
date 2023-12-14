
use common::read_input;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rock {
    Round,
    Cubed,
    Empty,
}

fn parse_panel(input: &str) -> Vec<Vec<Rock>> {
    use Rock::*;
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Round,
                    '#' => Cubed,
                    '.' => Empty,
                    c => panic!("unexpected char: {c}"),
                })
                .collect()
        })
        .collect()
}

fn round_coordinates(rocks: &[Vec<Rock>]) -> Vec<(usize, usize)> {
    (0..rocks.len())
        .flat_map(|x| (0..rocks[x].len()).map(move |y| (x, y)))
        .filter(|(x, y)| rocks[*x][*y] == Rock::Round)
        .collect_vec()
}

fn tilt_north(rocks: &mut [Vec<Rock>]) {
    let coordinates = round_coordinates(rocks);

    for (mut i, j) in coordinates {
        while i != 0 && rocks[i - 1][j] == Rock::Empty {
            rocks[i][j] = Rock::Empty;
            i -= 1;
            rocks[i][j] = Rock::Round;
        }
    }
}

#[allow(dead_code)]
fn display_rocks(panel: &[Vec<Rock>]) {
    for rocks in panel {
        for rock in rocks {
            match rock {
                Rock::Round => print!("O"),
                Rock::Cubed => print!("#"),
                Rock::Empty => print!("."),
            }
        }
        println!();
    }
}

fn value_rocks(mut panel: Vec<Vec<Rock>>) -> usize {
    panel.push(Vec::new());
    panel.reverse();
    panel
        .into_iter()
        .enumerate()
        .map(|(i, rocks)| {
            rocks
                .into_iter()
                .filter(|rock| rock == &Rock::Round)
                .count()
                * i
        })
        .sum()
}

fn part_1(mut panel: Vec<Vec<Rock>>) -> usize {
    tilt_north(&mut panel);
    value_rocks(panel)
}

fn apply_rock_cycle(
    panel: &mut [Vec<Rock>],
    mut coords: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    // north tilt
    coords.sort_by_key(|(i, _)| *i);
    let mut next_coords = vec![];

    for (mut i, j) in coords {
        while i != 0 && panel[i - 1][j] == Rock::Empty {
            panel[i][j] = Rock::Empty;
            i -= 1;
            panel[i][j] = Rock::Round;
        }
        next_coords.push((i, j));
    }

    // west tilt
    coords = next_coords;
    coords.sort_by_key(|(_, j)| *j);
    next_coords = vec![];
    for (i, mut j) in coords {
        while j != 0 && panel[i][j - 1] == Rock::Empty {
            panel[i][j] = Rock::Empty;
            j -= 1;
            panel[i][j] = Rock::Round;
        }
        next_coords.push((i, j));
    }

    // south tilt
    coords = next_coords;
    coords.sort_by_key(|(i, _)| -(*i as isize));
    next_coords = vec![];

    for (mut i, j) in coords {
        while i < panel.len() - 1 && panel[i + 1][j] == Rock::Empty {
            panel[i][j] = Rock::Empty;
            i += 1;
            panel[i][j] = Rock::Round;
        }
        next_coords.push((i, j));
    }

    // east tilt
    coords = next_coords;
    // bizarrely, this bugs out with a negative sort key
    // like with the south tilt
    coords.sort_by_key(|(_, j)| (*j));
    next_coords = vec![];

    for (i, mut j) in coords.into_iter().rev() {
        let len = panel[i].len() - 1;
        while j < len && panel[i][j + 1] == Rock::Empty {
            panel[i][j] = Rock::Empty;
            j += 1;
            panel[i][j] = Rock::Round;
        }
        next_coords.push((i, j));
    }

    next_coords
}

#[allow(dead_code)]
fn part_2(mut panel: Vec<Vec<Rock>>) -> usize {
    let magic_iter_count = 1000000000;
    let mut coords = round_coordinates(&panel);
    for _ in 1..=magic_iter_count {
        let old_coords = coords.clone();
        let new_coords = apply_rock_cycle(&mut panel, coords);
        coords = old_coords
            .into_iter()
            .zip(new_coords)
            .filter_map(|(old, new)| match old == new {
                true => None,
                false => Some(new),
            })
            .collect();
        if coords.is_empty() {
            break;
        }
        println!("{}", coords.len())
    }
    value_rocks(panel)
}

// part 2 isn't exactly complete
// Sample ouput: 136
fn main() {
    let input = read_input!();
    let panel = parse_panel(&input);
    let part_1 = part_1(panel.clone());
    // let part_2 = part_2(panel);
    println!("{part_1}")
}
