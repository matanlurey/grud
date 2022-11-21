use std::ops::{Index, IndexMut};

use crate::point::Point;

/// A [dense] fixed-size grid that stores elements using a [`Vec`].
///
/// [dense]: https://stackoverflow.com/questions/39030196/what-exactly-is-a-dense-array
pub struct Grid<T>
where
    T: Clone,
{
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone,
{
    /// Creates a new grid of the specified `width` and `height`, filling with `default`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let _ = Grid::new(3, 3, 0);
    /// ```
    pub fn new(width: usize, height: usize, default: T) -> Self {
        Self {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    /// Returns the width of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(2, 3, 0);
    /// assert_eq!(grid.width(), 2);
    /// ```
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(2, 3, 0);
    /// assert_eq!(grid.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the total size of the grid as represented by `width * height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(2, 3, 0);
    /// assert_eq!(grid.area(), 2 * 3);
    /// ```
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }
}

impl<T> Index<usize> for Grid<T>
where
    T: Clone,
{
    type Output = T;

    /// Given an index into the implementation vector, returns the underlying data.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(1, 1, "X");
    ///
    /// // Using a tuple to represent (width, height)
    /// assert_eq!(grid[0], "X");
    /// ```
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Grid<T>
where
    T: Clone,
{
    /// Given a two-dimensional coordinate [`Point`], sets the underlying data.
    ///
    /// # Examples
    ///
    /// Using a tuple to represent `(width, height)`:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let mut grid = Grid::new(1, 1, "X");
    ///
    /// // Using a tuple to represent (width, height)
    /// grid[(0, 0)] = "Y";
    ///
    /// # assert_eq!(grid[(0, 0)], "Y");
    /// ```
    ///
    /// Using a array to represent `[width, height]`:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let mut grid = Grid::new(1, 1, "X");
    ///
    /// grid[(0, 0)] = "Y";
    ///
    /// # assert_eq!(grid[[0, 0]], "Y");
    /// ```
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T, I> Index<I> for Grid<T>
where
    T: Clone,
    I: Point,
{
    type Output = T;

    /// Given a two-dimensional coordinate [`Point`], returns the underlying data.
    ///
    /// # Examples
    ///
    /// Using a tuple to represent `(width, height)`:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(1, 1, "X");
    ///
    /// assert_eq!(grid[(0, 0)], "X");
    /// ```
    ///
    /// Using an array to represent `[width, height]`:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(1, 1, "X");
    ///
    /// assert_eq!(grid[[0, 0]], "X");
    /// ```
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn index(&self, index: I) -> &Self::Output {
        let index = index.to_index(self.width());
        &self[index]
    }
}

impl<T, I> IndexMut<I> for Grid<T>
where
    T: Clone,
    I: Point,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.to_index(self.width());
        &mut self[index]
    }
}
