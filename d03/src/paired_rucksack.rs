use crate::Rucksack;

/// A rucksack defined by being split down the middle of each line.
pub struct PairedRucksack(Box<[u8]>);

impl Rucksack for PairedRucksack {
    fn duped_values(&self) -> u32 {
        // an attempt was made to improve performance with a set
        // however this degraded performance by a factor of six

        let half = self.0.len() / 2;
        let left = &self.0[..half];
        let right = &self.0[half..];

        for value in left {
            if right.contains(value) {
                return *value as u32;
            }
        }

        panic!("no duped value in paired rucksack")
    }

    fn into_vec(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let sack: Box<[u8]> = line.bytes().map(Self::priority).collect();
                Self(sack)
            })
            .collect()
    }
}
