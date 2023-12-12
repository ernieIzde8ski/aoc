use std::rc::Rc;

use common::read_input;
use itertools::Itertools;

// immutable pointers to arrays existing wherever
type List<T> = Rc<[T]>;

// Sum the distances between coordinates. Expands space.
fn sum_distances(
    coords: List<(isize, isize)>,
    empty_rows: List<isize>,
    empty_cols: List<isize>,
    hyperdimensional_folding_factor: isize,
) -> isize {
    let hyperdimensional_folding_factor = hyperdimensional_folding_factor - 1;
    let coords = coords
        .iter()
        .map(|(mut i, mut j)| {
            let row_incr = empty_rows.iter().filter(|n| **n < i).count();
            let col_incr = empty_cols.iter().filter(|n| **n < j).count();

            i += row_incr as isize * hyperdimensional_folding_factor;
            j += col_incr as isize * hyperdimensional_folding_factor;

            (i, j)
        })
        .combinations_with_replacement(2)
        .filter_map(|arr| match arr[0] == arr[1] {
            true => None,
            false => Some((arr[0], arr[1])),
        });

    let mut sum = 0;
    for (left, right) in coords {
        let s = isize::abs(right.1 - left.1) + isize::abs(right.0 - left.0);
        sum += s;
    }
    sum
}

// do we not feel the breath of empty space?
fn parse_space(input: &str) -> List<List<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

/// Grab the rows of Space which have no galaxies.
fn empty_rows<T: FromIterator<isize>>(space: List<List<char>>) -> T {
    space
        .iter()
        .enumerate()
        .filter_map(|(i, row)| match row.iter().any(|c| *c == '#') {
            false => Some(i as isize),
            true => None,
        })
        .collect()
}

/// Grab the columns of Space which have no galaxies.
fn empty_columns<T: FromIterator<isize>>(space: List<List<char>>) -> T {
    (0..space[0].len() as isize)
        .filter(|i| space.iter().map(|row| row[*i as usize]).all(|s| s == '.'))
        .collect()
}

/// Grab the (unexpanded) coordinates from Space.
fn coordinates<T: FromIterator<(isize, isize)>>(space: List<List<char>>) -> T {
    space
        .iter()
        .enumerate()
        .flat_map(|(i, chars)| {
            chars
                .iter()
                .enumerate()
                .map(move |(j, c)| (i as isize, j as isize, *c))
        })
        .filter_map(|(i, j, c)| match c {
            '#' => Some((i, j)),
            '.' => None,
            c => panic!("unexpected value: {c}"),
        })
        .collect()
}

// sample output: 374, 82000210
fn main() {
    let input = read_input!();

    let space = parse_space(&input);

    let empty_rows: List<isize> = empty_rows(space.clone());
    let empty_cols: List<isize> = empty_columns(space.clone());
    let coords: List<(isize, isize)> = coordinates(space);

    let p1_sum = sum_distances(coords.clone(), empty_rows.clone(), empty_cols.clone(), 2);
    let p2_sum = sum_distances(coords, empty_rows, empty_cols, 1_000_000);
    println!("{p1_sum}, {p2_sum}")
}
