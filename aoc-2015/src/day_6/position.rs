use std::fmt::Display;

/// Implementation of a Position in a 2D space.
///
///  Y
///  ^
///  |
///  .--> X
/// 0,0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Horizontal axis
    x: u32,

    // Vertical axis
    y: u32,
}

impl Position {
    /// Create a new Position at (`x`, `y`).
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Return the value on the horizontal X axis.
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Return the value on the vertical Y axis.
    pub fn y(&self) -> u32 {
        self.y
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();

        Position::new(x.parse().unwrap(), y.parse().unwrap())
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
