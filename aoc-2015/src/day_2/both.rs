/// Present structure.
#[derive(Debug)]
pub struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    /// Create a new [Present] with `lenght`, `width` and `height`.
    pub fn new(length: u32, width: u32, height: u32) -> Self {
        Present {
            length,
            width,
            height,
        }
    }

    /// Return the surface area of the present.
    pub fn surface_area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    /// Return the area of the smallest side.
    pub fn smallest_area(&self) -> u32 {
        [
            self.length * self.width,
            self.length * self.height,
            self.width * self.height,
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    /// Return the smallest perimeter of any one face.
    pub fn smallest_perimeter(&self) -> u32 {
        [
            2 * (self.length + self.width),
            2 * (self.length + self.height),
            2 * (self.width + self.height),
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    /// Return the cubic feet of volume.
    pub fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

impl From<&str> for Present {
    fn from(value: &str) -> Self {
        let values: Vec<&str> = value.split('x').collect();

        assert_eq!(values.len(), 3);

        Present::new(
            values[0].parse().unwrap(),
            values[1].parse().unwrap(),
            values[2].parse().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_area() {
        // Given
        let present = Present::new(2, 3, 4);

        // When
        let area = present.surface_area();

        // Then
        assert_eq!(area, 52);
    }

    #[test]
    fn smallest_area() {
        // Given
        let present = Present::new(2, 3, 4);

        // When
        let area = present.smallest_area();

        // Then
        assert_eq!(area, 6);
    }

    #[test]
    fn smallest_perimeter() {
        // Given
        let present = Present::new(2, 3, 4);

        // When
        let area = present.smallest_perimeter();

        // Then
        assert_eq!(area, 10);
    }

    #[test]
    fn volume() {
        // Given
        let present = Present::new(2, 3, 4);

        // When
        let area = present.volume();

        // Then
        assert_eq!(area, 2 * 3 * 4);
    }
}
