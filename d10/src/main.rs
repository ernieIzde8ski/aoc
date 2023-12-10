pub mod coord;
pub mod diagram;

use anyhow::{bail, Result};

use common::read_input;

use coord::Coord;
use diagram::Diagram;

// Only did part 1, sorry
// Can't quite wrap my head around how to do part 2
// Sample output: 80
fn main() -> Result<()> {
    let input = read_input!();
    let diagram = Diagram::from(input.as_ref());

    let starting_coord = diagram.find_s_coord()?;

    let mut coord_0 = starting_coord;
    let mut coord_1: Coord = 'def_c1: {
        macro_rules! case {
            ($i:expr, $j:expr, $mat:pat) => {{
                let coord = (coord_0.0 + $i, coord_0.1 + $j);
                match diagram[coord] {
                    $mat => break 'def_c1 coord,
                    _ => {}
                }
            }};
        }
        // top case
        if coord_0.0 != 0 {
            case!(-1, 0, '7' | '|' | 'F');
        }
        // bottom case
        if (coord_0.0 + 1) as usize != diagram.len() {
            case!(1, 0, 'J' | '|' | 'L')
        }
        // left case
        if coord_0.1 != 0 {
            case!(0, -1, 'L' | '-' | 'F');
        }
        // right case
        if (coord_0.1 + 1) as usize != diagram[0].len() {
            case!(0, 1, '7' | '-' | 'J')
        }

        panic!("oopsie doopsie your input is fucky wucky")
    };

    let mut path: Vec<Coord> = Vec::new();

    while coord_1 != starting_coord {
        let coord_diff = coord::sub(coord_0, coord_1);

        let coord_2: Coord = match (coord_diff, diagram[coord_1]) {
            ((0, -1), 'J') | ((0, 1), 'L') => (-1, 0),
            ((0, -1), '7') | ((0, 1), 'F') => (1, 0),
            ((-1, 0), 'J') | ((1, 0), '7') => (0, -1),
            ((-1, 0), 'L') | ((1, 0), 'F') => (0, 1),
            ((0, n), '-') => (0, -n),
            ((n, 0), '|') => (-n, 0),
            (a, b) => bail!("{coord_1:?}, {a:?}, {b}"),
        };
        let coord_2 = coord::add(coord_1, coord_2);

        path.push(coord_0);
        coord_0 = coord_1;
        coord_1 = coord_2;
    }
    path.push(coord_1);

    Ok(println!("{:?}", path.len() / 2))
}
