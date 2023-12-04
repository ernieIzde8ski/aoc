use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Span {
    pub x: usize,
    pub y: RangeInclusive<usize>,
    pub value: u32,
}

impl Span {
    pub fn new(x: usize, y: RangeInclusive<usize>, value: u32) -> Self {
        Self { x, y, value }
    }
}
