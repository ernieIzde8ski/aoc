pub type Coord = (isize, isize);

pub fn sub(left: Coord, right: Coord) -> Coord {
    (left.0 - right.0, left.1 - right.1)
}

pub fn add(left: Coord, right: Coord) -> Coord {
    (left.0 + right.0, left.1 + right.1)
}
