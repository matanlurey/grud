//! Default implementation of a 2-dimensional grid of elements.
//!
//! See [`Grid`] for details.

use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use crate::point::Point;

/// A [dense] fixed-size grid that stores elements using a [`Vec`].
///
/// [dense]: https://stackoverflow.com/questions/39030196/what-exactly-is-a-dense-array
#[derive(Clone)]
pub struct Grid<T>
where
    T: Clone,
{
    data: Vec<T>,
    width: usize,
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
        }
    }

    /// Creates a new grid of the specified `width`, inferring height from the length of the `data`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::with_width(2, vec![1, 2, 3, 4, 5, 6]);
    /// assert_eq!(grid.width(), 2);
    /// assert_eq!(grid.height(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// If `data.len()` is not evenly divisble by `width`.
    pub fn with_width(width: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len() % width,
            0,
            "Data length {} not divisible by {width}",
            data.len()
        );
        Self { data, width }
    }

    /// Returns the grid represnted as a flattened 2-dimensional vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::from(vec![
    ///   vec![1, 2],
    ///   vec![3, 4],
    ///   vec![5, 6],
    /// ]);
    ///
    /// assert_eq!(grid.as_vec(), &vec![1, 2, 3, 4, 5, 6]);
    /// ```
    pub fn as_vec(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns the grid represnted by a multi-dimensional matrix (i.e. vector of vectors).
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::with_width(2, vec![1, 2, 3, 4, 5, 6]);
    /// assert_eq!(grid.to_matrix(), vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    ///     vec![5, 6],
    /// ]);
    /// ```
    pub fn to_matrix(&self) -> Vec<Vec<T>> {
        let mut data = Vec::<Vec<T>>::with_capacity(self.height());
        for j in 0..self.height() {
            let mut row = Vec::<T>::with_capacity(self.width());
            for i in 0..self.width() {
                row.push(self[(i, j)].clone());
            }
            data.push(row);
        }
        data
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
        self.data.len() / self.width()
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

impl<T> Debug for Grid<T>
where
    T: Clone + Debug,
{
    /// Formats the grid into string output for debugging.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("data", &self.data)
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

impl<T> Display for Grid<T>
where
    T: Clone + Display,
{
    /// Formats the grid into a multi-line string output.
    ///
    /// If `T` is [`Display`] and is represented by a consistent sized grapheme cluster, the effect
    /// is similar to using a text-based user interface to output grahaeme clusters in a 2D grid:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::with_width(3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    ///
    /// // 123
    /// // 456
    /// // 789
    /// assert_eq!(format!("{}", grid), "123\n456\n789\n");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.height() {
            for i in 0..self.width() {
                write!(f, "{}", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
    T: Clone,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns an iterator that walks the grid in indexed order.
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::with_width(2, vec!["a", "b", "c", "d"]);
    ///
    /// let items: Vec<String> = grid.into_iter().map(|i| i.to_ascii_uppercase()).collect();
    ///
    /// assert_eq!(items, vec!["A", "B", "C", "D"]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T>
where
    T: Clone,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    /// Returns an iterator that walks the grid in indexed order with mutable references.
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let mut grid = Grid::with_width(2, vec![1, 2, 3, 4]);
    ///
    /// for i in &mut grid {
    ///   *i += 1;
    /// }
    ///
    /// assert_eq!(grid.as_vec(), &vec![2, 3, 4, 5]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T>
where
    T: Clone,
{
    /// Converts a multi-dimensional vector of vectors into a [`Grid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::from(vec![
    ///   vec![1, 2],
    ///   vec![3, 4],
    ///   vec![5, 6],
    /// ]);
    ///
    /// assert_eq!(grid.width(), 2);
    /// assert_eq!(grid.height(), 3);
    /// ```
    ///
    /// # Panics
    ///
    /// If the length of every inner vector is not the same.
    fn from(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        if height == 0 {
            return Self {
                data: vec![],
                width: 0,
            };
        }
        let width = data[0].len();
        assert!(
            data.iter().all(|v| v.len() == width),
            "Length of every inner vector not the smae"
        );
        Self {
            data: data.iter().flat_map(|v| v.clone()).collect(),
            width,
        }
    }
}

impl<T> Index<usize> for Grid<T>
where
    T: Clone,
{
    type Output = T;

    /// Given an index into the implementation vector, returns the underlying data.
    ///
    /// This operator requires understanding the internal representation of data. For example,
    /// a 3x3 Grid (i.e. `Grid::new(3, 3, "•")`) has the indexed locations laid out as such:
    ///
    /// ```txt
    /// •0 •1 •2
    /// •3 •4 •5
    /// •6 •7 •8
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(1, 1, "X");
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
    /// Given an index into the implementation vector, sets the underlying data.
    ///
    /// This operator requires understanding the internal representation of data. For example,
    /// a 3x3 Grid (i.e. `Grid::new(3, 3, "•")`) has the indexed locations laid out as such:
    ///
    /// ```txt
    /// •0 •1 •2
    /// •3 •4 •5
    /// •6 •7 •8
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let mut grid = Grid::new(1, 1, "X");
    /// grid[0] = "Y";
    ///
    /// # assert_eq!(grid[0], "Y");
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
    /// assert_eq!(grid[(0, 0)], "X");
    /// ```
    ///
    /// Using an array to represent `[width, height]`:
    ///
    /// ```
    /// use grud::Grid;
    ///
    /// let grid = Grid::new(1, 1, "X");
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
    /// grid[(0, 0)] = "Y";
    ///
    /// # assert_eq!(grid[[0, 0]], "Y");
    /// ```
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let index = index.to_index(self.width());
        &mut self[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid() {
        let grid = Grid::new(2, 3, " ");

        assert_eq!(grid.area(), 6);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.as_vec(), &vec![" ", " ", " ", " ", " ", " "]);
    }

    #[test]
    fn new_grid_of_width_to_matrix() {
        let grid = Grid::with_width(2, vec!["A", "B", "C", "D"]);

        assert_eq!(grid.to_matrix(), vec![vec!["A", "B"], vec!["C", "D"]])
    }

    #[test]
    #[should_panic]
    fn new_grid_of_width_not_divisible() {
        Grid::with_width(2, vec![1, 2, 3]);
    }

    #[test]
    fn grid_from_matrix() {
        let grid: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();

        assert_eq!(grid.as_vec(), &vec!["A", "B", "C", "D"]);
    }

    #[test]
    #[should_panic]
    fn grid_from_matrix_not_consistent() {
        let _: Grid<_> = vec![vec!["A"], vec!["B", "C"]].into();
    }

    #[test]
    fn grid_clone() {
        let a: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();
        let b = a.clone();

        assert_eq!(a.as_vec(), b.as_vec());
    }

    #[test]
    fn grid_debug() {
        let a: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();
        let a = format!("{:?}", a);

        assert_eq!(
            a,
            "Grid { data: [\"A\", \"B\", \"C\", \"D\"], width: 2, height: 2 }"
        );
    }

    #[test]
    fn grid_display() {
        let a: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();
        let a = format!("{}", a);

        assert_eq!(a, "AB\nCD\n");
    }

    #[test]
    fn grid_iter() {
        let a: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();
        let v = a
            .into_iter()
            .map(|i| i.to_ascii_lowercase())
            .collect::<Vec<String>>();

        assert_eq!(v, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn grid_iter_mut() {
        let mut a: Grid<_> = vec![vec![1, 2], vec![3, 4]].into();
        for i in &mut a {
            *i += 1;
        }

        assert_eq!(a.as_vec(), &vec![2, 3, 4, 5]);
    }

    #[test]
    fn grid_index() {
        let grid: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();

        assert_eq!(grid[0], "A");
        assert_eq!(grid[1], "B");
        assert_eq!(grid[2], "C");
        assert_eq!(grid[3], "D");
    }

    #[test]
    #[should_panic]
    fn grid_index_out_of_bounds() {
        let grid: Grid<()> = vec![].into();

        #[allow(clippy::no_effect)]
        grid[0];
    }

    #[test]
    fn grid_mut_index() {
        let mut grid: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();

        grid[0] = "a";
        assert_eq!(grid[0], "a");
        assert_eq!(grid[1], "B");
        assert_eq!(grid[2], "C");
        assert_eq!(grid[3], "D");
    }

    #[test]
    #[should_panic]
    fn grid_mut_index_out_of_bounds() {
        let mut grid: Grid<()> = vec![].into();

        grid[0] = ();
    }

    #[test]
    fn grid_index_point() {
        let grid: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();

        assert_eq!(grid[(0, 0)], "A");
        assert_eq!(grid[(1, 0)], "B");
        assert_eq!(grid[(0, 1)], "C");
        assert_eq!(grid[(1, 1)], "D");
    }

    #[test]
    fn grid_mut_index_point() {
        let mut grid: Grid<_> = vec![vec!["A", "B"], vec!["C", "D"]].into();

        grid[(0, 0)] = "a";
        grid[(1, 0)] = "b";
        grid[(0, 1)] = "c";
        grid[(1, 1)] = "d";

        assert_eq!(grid.as_vec(), &vec!["a", "b", "c", "d"]);
    }
}
