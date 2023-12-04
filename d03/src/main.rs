mod part;
mod schematic;
mod span;

use anyhow::Result;
use common::read_input;

use part::Part;
use schematic::Schematic;
use span::Span;

// Expected sample output:
//      4361
//      thread 'main' panicked... not implemented
fn main() -> Result<()> {
    let input = read_input!();
    let schematic: Schematic = Schematic::from(&*input);
    println!("{}", schematic.part_numbers());
    println!("{}", schematic.gear_ratios());
    Ok(())
}
