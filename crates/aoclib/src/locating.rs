use std::ops::Add;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position64(pub i64, pub i64);

impl Add<Direction> for Position64 {
    type Output = Position64;

    fn add(self, dir: Direction) -> Self::Output {
        let dir = dir.unit();
        Position64(self.0 + dir.0, self.1 + dir.1)
    }
}

impl Position64 {
    pub fn distance_to(&self, other: &Position64) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    #[default]
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn unit(&self) -> (i64, i64) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

/*
#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Direction64(i64, i64);

pub const UP : Direction64 = Direction64(-1,0);
pub const DOWN : Direction64 = Direction64(-1,0);
pub const LEFT : Direction64 = Direction64(-1,0);
pub const RIGHT : Direction64 = Direction64(-1,0);

impl Direction64 {
    pub fn left(&self) -> Self {
        Direction64(self.1, -self.0)
    }

    pub fn right(&self) -> Self {
        Direction64(-self.1, self.0)
    }
}
*/
