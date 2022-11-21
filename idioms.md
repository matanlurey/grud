# Rust Idioms I learned from other Crates

I'm not sure which of these is truly idiomatic yet, but let's dive in:

## blitter

Source: <https://crates.io/crates/blitter>

Blitter was interesting because it's _really_ a (32-bit) framebuffer, and not
a true (generic) representation of a two-dimensional grid.

However, it was neat to see how various bitmap operations could be performed in
Rust; I'll likely want some subset/superset of these operations within my own
crate (either by default or in another library).

```rs
struct FrameBuffer<'a> {
    pub width: usize,
    pub height: usize,

    // Does this mean that the struct is always mutable? Or something else?
    pub pixels: &'a mut Vec<u32>,
}
```

Some interesting methods:

- [`clear`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#312-316)
- [`clear_area`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#288-341)
- [`draw_pixel`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#319-325)
- [`draw_fatpixel`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#328-340)

```rs
pub struct Bitmap<'a> {
    pub w: usize,
    pub h: usize,
    pub x: isize,
    pub y: isize,
    pub pixels: &'a Vec<u32>,
}
```

Some interesting methods:

- [`blit`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#140-142)
- [`blit_mask`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#119-137)
- [`blit_part`](https://docs.rs/blitter/0.6.1/src/blitter/lib.rs.html#145-163)

## das-grid

Source: <https://crates.io/crates/das-grid>

This one was neat, because it looks like it was built with games in mind.

```rs
// This was interesting, one of my earlier prototypes was similar.
pub struct Grid<T: Copy + Clone> { /* fields omitted */ }
```

Some factory methods:

- [`new`](https://docs.rs/das-grid/0.1.5/src/das_grid/lib.rs.html#220-237)
- [`new_from_vector`](https://docs.rs/das-grid/0.1.5/src/das_grid/lib.rs.html#245-267)

Some interesting methods:

- [`get_subgrid`](https://docs.rs/das-grid/0.1.5/src/das_grid/lib.rs.html#314-330)
- [`stamp_subgrid`](https://docs.rs/das-grid/0.1.5/src/das_grid/lib.rs.html#283-301)

## gridit

Source: <https://crates.io/crates/gridit>

Gridit had some interesting methods to help with movement patterns:

- [`gridit::iter`](https://docs.rs/gridit/0.1.0/gridit/iter/index.html)
- [`gridit::pattern`](https://docs.rs/gridit/0.1.0/gridit/pattern/index.html)

## simple-grid

Source: <https://crates.io/crates/simple-grid>

This crate had the most amount of built-in methods, by far. Some interesting:

- [`flip_horizontally`](https://docs.rs/simple-grid/2.1.1/src/simple_grid/lib.rs.html#733-749)
- [`pop_row`](https://docs.rs/simple-grid/2.1.1/src/simple_grid/lib.rs.html#404-410)
- [`rotate_ccw`](https://docs.rs/simple-grid/2.1.1/simple_grid/struct.Grid.html#method.rotate_ccw)
- [`subgrid`](https://docs.rs/simple-grid/2.1.1/src/simple_grid/lib.rs.html#113-145)
- [`to_pretty_string`](https://docs.rs/simple-grid/2.1.1/simple_grid/struct.Grid.html#method.to_pretty_string)
- [`transpose`](https://docs.rs/simple-grid/2.1.1/simple_grid/struct.Grid.html#method.transpose)

## sark_grids

Source: <https://crates.io/crates/sark_grids>

This crate seemed like the one that thought the most about performance.

It probably deserves the most "deeper" look once I have something working:

- [`sark_grids::directions`](https://docs.rs/sark_grids/0.5.5/sark_grids/directions/index.html)
- [`sark_grids::geometry`](https://docs.rs/sark_grids/0.5.5/sark_grids/geometry/index.html)
- [`Canvas`](https://docs.rs/sark_grids/0.5.5/src/sark_grids/util/canvas.rs.html#7-10)
- [`SparseGrid`](https://docs.rs/sark_grids/0.5.5/src/sark_grids/sparse_grid.rs.html#40-43)
- [`WorldGrid`](https://docs.rs/sark_grids/0.5.5/src/sark_grids/world_grid.rs.html#1-303)

## tapestry

Source: <https://crates.io/crates/tapestry>

This crate seems the least maintained, but had some interesting utilities:

- [`ClusterLayers`](https://docs.rs/tapestry/0.1.0/src/tapestry/patterns.rs.html#134-152)
- [`FloodIter`](https://docs.rs/tapestry/0.1.0/src/tapestry/grid.rs.html#264-269)
