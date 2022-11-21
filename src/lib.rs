//! Store and access data in two-dimensional grids.
//!
//! Simple use-cases can use the exported paths directly:
//!
//! ```
//! use grud::Grid;
//!
//! let _ = Grid::new(2, 4, " ");
//! ```
//!
//! Other modules are included for additional functionality.

pub mod grid;
pub mod point;

pub use grid::Grid;

pub mod prelude {
    //! Most used paths within Grud, that can be imported easily.
    //!
    //! ```
    //! use grud::prelude::*;
    //!
    //! fn uses_point(p: impl Point) {
    //!     assert_eq!(p.x(), 2);
    //!     assert_eq!(p.y(), 4);
    //! }
    //!
    //! uses_point((2, 4));
    //! uses_point([2, 4]);
    //! ```

    pub use crate::grid::Grid;
    pub use crate::point::Point;
}
