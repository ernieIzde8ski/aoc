use crate::{Part, Span};

use std::ops::Deref;

pub struct Schematic(Vec<Vec<Part>>);

impl Deref for Schematic {
    type Target = [Vec<Part>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut schematic = vec![];
        for line in value.lines() {
            let mut schematic_line = vec![];
            for c in line.chars() {
                let v = match c.to_digit(10) {
                    Some(d) => Part::Digit(d),
                    None if c != '.' => Part::Symbol(c),
                    None => Part::Empty,
                };
                schematic_line.push(v);
            }
            schematic.push(schematic_line);
        }
        Self(schematic)
    }
}

fn digits_to_number(digits: &[u32]) -> u32 {
    let mut sum = 0;
    for (pow, digit) in digits.iter().rev().enumerate() {
        sum += digit * u32::pow(10, pow as u32);
    }
    sum
}

impl Schematic {
    pub fn spans(&self) -> Vec<Span> {
        let mut resp = vec![];
        for x in 0..self.len() {
            let mut cols = 0..self[x].len();
            while let Some(y_0) = cols.next() {
                let mut y_1 = y_0;
                let mut digits = vec![];

                while let Part::Digit(d) = self[x][y_1] {
                    digits.push(d);
                    match cols.next() {
                        Some(y) => y_1 = y,
                        None => {
                            y_1 += 1;
                            break;
                        }
                    }
                }

                if !digits.is_empty() {
                    let value = digits_to_number(&digits);
                    let span = Span::new(x, y_0..=y_1 - 1, value);
                    resp.push(span)
                }
            }
        }
        resp
    }
    pub fn surrounding_coords(&self, span: &Span) -> Vec<(usize, usize)> {
        let mut resp = vec![];

        let start = *span.y.start();

        let y_0 = match start {
            0 => 0,
            mut n => {
                n -= 1;
                resp.push((span.x, n));
                n
            }
        };
        let y_1 = {
            let y = span.y.end() + 1;
            match y == self[span.x].len() {
                true => y - 1,
                false => {
                    resp.push((span.x, y));
                    y
                }
            }
        };

        // above and beneath the span
        if span.x != 0 {
            let x = span.x - 1;
            resp.extend((y_0..=y_1).map(|i| (x, i)));
        };
        if span.x != self.len() - 1 {
            let x = span.x + 1;
            resp.extend((y_0..=y_1).map(|i| (x, i)))
        };
        resp
    }
    pub fn part_numbers(&self) -> u32 {
        self.spans()
            .iter()
            .filter(|span| {
                self.surrounding_coords(span)
                    .iter()
                    .any(|(x, y)| matches!(self[*x][*y], Part::Symbol(_)))
            })
            .map(|span| span.value)
            // .inspect(|n| eprintln!("Adding number: {n}"))
            .sum()
    }
    pub fn gear_ratios(&self) -> u32 {
        unimplemented!("I cannot be assed.")
    }
}
