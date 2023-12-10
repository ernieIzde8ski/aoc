use anyhow::{anyhow, Result};

use crate::Coord;
use std::ops::{Deref, Index};

pub struct Diagram(Vec<Vec<char>>);

impl Diagram {
    pub fn find_s_coord(&self) -> Result<Coord> {
        for i in 0..self.len() {
            for j in 0..self.0[i].len() {
                if self.0[i][j] == 'S' {
                    return Ok((i as isize, j as isize));
                }
            }
        }

        Err(anyhow!("couldn't find S char!"))
    }
}

impl Deref for Diagram {
    type Target = [Vec<char>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Index<(isize, isize)> for Diagram {
    type Output = char;

    fn index(&self, coord: (isize, isize)) -> &Self::Output {
        &self.0[coord.0 as usize][coord.1 as usize]
    }
}

impl Index<usize> for Diagram {
    type Output = [char];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl From<Vec<Vec<char>>> for Diagram {
    fn from(value: Vec<Vec<char>>) -> Self {
        Self(value)
    }
}

impl From<&str> for Diagram {
    fn from(value: &str) -> Self {
        value
            .lines()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<char>>>()
            .into()
    }
}
