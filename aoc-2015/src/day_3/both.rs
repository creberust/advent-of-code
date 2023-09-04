/// Position in a 2 dimensional space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Create a new Position at (x, y).
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

/// Cardinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        use Direction::*;

        match value {
            '^' => North,
            'v' => South,
            '>' => East,
            '<' => West,
            value => panic!("invalid value for direction: {}", value),
        }
    }
}
