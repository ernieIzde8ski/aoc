use std::ops::RangeInclusive;

#[derive(Debug)]
struct CampPair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

#[derive(Debug)]
struct Overlaps {
    full_overlaps: u32,
    partial_overlaps: u32,
}

impl CampPair {
    /// Whether one pair contains the other.
    fn full_overlap(&self) -> bool {
        (self.left.contains(self.right.start()) && self.left.contains(self.right.end()))
            || (self.right.contains(self.left.start()) && self.right.contains(self.left.end()))
    }

    /// Whether one pair has at least one element in common with the other.
    fn partial_overlap(&self) -> bool {
        self.left.contains(self.right.start()) || self.right.contains(self.left.start())
    }

    fn new(left: RangeInclusive<u32>, right: RangeInclusive<u32>) -> Self {
        Self { left, right }
    }

    /// Creates a new vector of Self from the input file.
    fn new_vec(value: &str) -> Vec<Self> {
        value
            .lines()
            .map(|l| {
                let (left, right) = l.split_once(',').unwrap();

                let left_range: (u32, u32) = {
                    let (lower, upper) = left.split_once('-').unwrap();
                    (lower.parse().unwrap(), upper.parse().unwrap())
                };

                let right_range: (u32, u32) = {
                    let (lower, upper) = right.split_once('-').unwrap();
                    (lower.parse().unwrap(), upper.parse().unwrap())
                };

                CampPair::new(left_range.0..=left_range.1, right_range.0..=right_range.1)
            })
            .collect()
    }

    fn total_overlapping(pairs: &[Self]) -> Overlaps {
        let mut full_overlaps = 0;
        let mut partial_overlaps = 0;
        for pair in pairs {
            if pair.full_overlap() {
                full_overlaps += 1;
                partial_overlaps += 1;
            } else if pair.partial_overlap() {
                partial_overlaps += 1;
            };
        }
        Overlaps {
            full_overlaps,
            partial_overlaps,
        }
    }
}

fn main() {
    let input = common::read_input!();

    let pairs = CampPair::new_vec(&input);
    let overlaps = CampPair::total_overlapping(&pairs);

    println!("Total fully overlapping:  {}", overlaps.full_overlaps);
    println!("Total partly overlapping: {}", overlaps.partial_overlaps);
}
