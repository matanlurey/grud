//! Store and access data in two-dimensional grids.

pub mod grid;
pub mod point;

pub use grid::Grid;

pub mod prelude {
    pub use crate::grid::Grid;
    pub use crate::point::Point;
}
