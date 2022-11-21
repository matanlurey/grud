//! TODO: Document.

/// A 2-dimensional point.
pub trait Point: Clone + Copy {
    /// Returns the x-coordinate.
    fn x(&self) -> usize;

    /// Returns the y-coordinate.
    fn y(&self) -> usize;

    /// Given the `width` of a grid, converts to an index into a 2-dimensional space (e.g. [`Vec`]).
    fn to_index(&self, width: usize) -> usize {
        self.y() * width + self.x()
    }
}

impl Point for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }
}

impl Point for [usize; 2] {
    fn x(&self) -> usize {
        self[0]
    }

    fn y(&self) -> usize {
        self[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_as_point() {
        let point = (1, 2);
        assert_eq!(point.x(), 1);
        assert_eq!(point.y(), 2);
        assert_eq!(point.to_index(2), 5);
    }

    #[test]
    fn array_as_point() {
        let point = [1, 2];
        assert_eq!(point.x(), 1);
        assert_eq!(point.y(), 2);
        assert_eq!(point.to_index(2), 5);
    }
}
