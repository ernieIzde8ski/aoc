use std::rc::Rc;

use common::read_input;

/// Finds the first index after which the preceding N characters
/// contain no repeats. Returns this index.
fn find_marker(chars: &[char], max_repeats: usize) -> usize {
    // i..j is any range checked for a marker
    'ranges: for i in 0.. {
        let j = i + max_repeats;
        // `l` and `r` are optimal elements to compare in the i..j
        // range. First & second, ..., first & last, second & third,
        // ..., second-to-last & last.

        // They could be slightly optimized further (note that checks
        // will be rerun on the next `i=` value), but I don't care enough.
        for l in i..(j - 1) {
            for r in (l + 1)..j {
                if chars[l] == chars[r] {
                    continue 'ranges;
                }
            }
        }

        return j;
    }

    panic!("This should never occur")
}

/// Sample output:
///     start-of-packet marker:  7
///     start-of-message marker: 19
fn main() {
    let input = read_input!();
    let chars: Rc<[char]> = input.chars().collect();

    println!("start-of-packet marker:  {}", find_marker(&chars, 4));
    println!("start-of-message marker: {}", find_marker(&chars, 14));
}
