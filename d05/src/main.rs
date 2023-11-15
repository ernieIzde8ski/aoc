use common::read_input;

#[derive(Clone, Debug)]
struct MoveInstruction {
    total: u8,
    from: u8,
    to: u8,
}

#[derive(Clone, Debug)]
struct Input {
    /// Some stacks of crates.
    stacks: Vec<Vec<u8>>,
    /// Instructions on crate stack movement.
    moves: Vec<MoveInstruction>,
}

impl Input {
    fn parse_input_file() -> Self {
        let lines = read_input!();
        let mut lines = lines.lines();

        let stacks = {
            // TODO: take_while
            let mut crate_lines = Vec::new();
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    break;
                }
                crate_lines.push(line.as_bytes());
            }

            // last line doesn't actually contain information for us
            // just names the crate indeces
            crate_lines.pop();

            let stacks_total = (crate_lines[0].len() + 1) / 4;
            let stack_size = crate_lines.len();

            let mut stacks = Vec::new();
            for i in 0..stacks_total {
                let mut stack = Vec::new();
                let y = i * 4 + 1;
                for x in (0..(stack_size)).rev() {
                    match crate_lines[x][y] {
                        // when we hit a space (ascii 32), there will
                        // be no crates afterward on this stack
                        32 => break,
                        b => stack.push(b),
                    }
                }
                stacks.push(stack);
            }
            stacks
        };

        let moves = lines
            .map(|l| {
                let mut numbers = l.split(' ').filter_map(|s| s.parse::<u8>().ok());

                // the file itself is 1-indexed
                MoveInstruction {
                    total: numbers.next().unwrap(),
                    from: numbers.next().unwrap() - 1,
                    to: numbers.next().unwrap() - 1,
                }
            })
            .collect();

        Self { stacks, moves }
    }

    /// First half of the puzzle. Sort crates one-by-one (supposedly).
    fn sort_individually(&mut self) {
        for mv in self.moves.iter() {
            let loser = &mut self.stacks[mv.from as usize];
            let mut plunder = loser.split_off(loser.len() - (mv.total as usize));
            plunder.reverse();
            self.stacks[mv.to as usize].extend(plunder);
        }
    }

    /// Second half of the puzzle. Sorts crates in bulk.
    /// (It's the same code without a reverse.)
    fn sort_by_bulk(&mut self) {
        for mv in self.moves.iter() {
            let loser = &mut self.stacks[mv.from as usize];
            let plunder = loser.split_off(loser.len() - (mv.total as usize));
            self.stacks[mv.to as usize].extend(plunder);
        }
    }

    /// Returns the top character from each byte
    fn crate_tops(&mut self) -> String {
        self.stacks.iter().map(|c| c[c.len() - 1] as char).collect()
    }
}

fn main() {
    let mut input_0 = Input::parse_input_file();
    let mut input_1 = input_0.clone();
    input_0.sort_individually();
    input_1.sort_by_bulk();
    println!("Part 1 tops: {}", input_0.crate_tops());
    println!("Part 2 tops: {}", input_1.crate_tops());
}
