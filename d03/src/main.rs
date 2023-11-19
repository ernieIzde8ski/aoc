mod paired_rucksack;
mod triplet_rucksack;

use common::read_input;

use crate::{paired_rucksack::PairedRucksack, triplet_rucksack::TripletRucksack};

trait Rucksack: Sized {
    /// Helper function for converting a `u8` (representing an ASCII
    /// char) to the "priority" type. While this would be an issue if
    /// we expected *any* valid UTF-8, the input.txt given only
    /// contains valid ASCII. The bitwise operation reduces the time
    /// this function takes by a factor of like three or four.
    fn priority(ch: u8) -> u8 {
        (ch & 0b00011111) + (26 * ((!ch >> 5) & 1))
    }

    /// Takes all Self::duped_values in a Vec<Self> and sums them together.
    fn sum(input: &[Self]) -> u32 {
        // no noticeable difference was noted from expanding this out
        // nor from turning the return type of duped_values into u8
        input.iter().map(|sack| sack.duped_values()).sum()
    }

    fn duped_values(&self) -> u32;
    fn into_vec(input: &str) -> Vec<Self>;
}

fn main() {
    let input = read_input!();

    let paired_rucksack = PairedRucksack::into_vec(&input);
    let triplet_rucksack = TripletRucksack::into_vec(&input);

    let paired_sum = PairedRucksack::sum(&paired_rucksack);
    let triplet_sum = TripletRucksack::sum(&triplet_rucksack);

    println!("Paired sack sum:  {paired_sum}");
    println!("Triplet sack sum: {triplet_sum}");
}
