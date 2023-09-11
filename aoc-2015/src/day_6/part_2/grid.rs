use crate::day_6::position::Position;

const SIZE: u32 = 1000;
const GRID_SIZE: u32 = SIZE * SIZE;

/// Implementation of a Grid for par
pub struct Grid {
    data: Vec<u32>,
}

impl Grid {
    /// Create a new Grid.
    pub fn new() -> Self {
        Self {
            data: vec![0; GRID_SIZE as usize],
        }
    }

    /// Increase the value by 1 at `coord`.
    pub fn increase(&mut self, coord: Position) {
        *self
            .data
            .get_mut((coord.x() + coord.y() * SIZE) as usize)
            .unwrap() += 1;
    }

    /// Decrease the value by 1 at `coord`.
    pub fn decrease(&mut self, coord: Position) {
        let value = self
            .data
            .get_mut((coord.x() + coord.y() * SIZE) as usize)
            .unwrap();

        if *value != 0 {
            *value -= 1;
        }
    }

    /// Return the total number of brightness accross all the lights.
    pub fn total_brightness(&self) -> u64 {
        self.data.iter().sum::<u32>() as u64
    }
}
