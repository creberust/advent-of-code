use super::position::Position;

const SIZE: u32 = 1000;
const GRID_SIZE: u32 = SIZE * SIZE;

/// Grid implementation.
#[derive(Debug)]
pub struct Grid {
    /// The data stored on the grid.
    data: [bool; GRID_SIZE as usize],
}

impl Grid {
    /// Create a new grid with every cell set to off.
    pub fn new() -> Self {
        Self {
            data: [false; GRID_SIZE as usize],
        }
    }

    /// Return the number of light lit.
    pub fn lit(&self) -> u64 {
        self.data
            .iter()
            .fold(0, |previous, cell| previous + if *cell { 1 } else { 0 })
    }

    /// Set the cell at `coord` the specified `value`.
    pub fn set(&mut self, coord: Position, value: bool) {
        *self
            .data
            .get_mut((coord.x() + coord.y() * SIZE) as usize)
            .unwrap() = value;
    }

    /// Toggle the value of the cell at `coord`.
    pub fn toggle(&mut self, coord: Position) {
        let value = self
            .data
            .get_mut((coord.x() + coord.y() * SIZE) as usize)
            .unwrap();

        *value = !*value;
    }
}
