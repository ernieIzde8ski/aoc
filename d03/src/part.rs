#[derive(Debug)]
pub enum Part {
    Digit(u32),
    Symbol(char),
    Empty,
}
