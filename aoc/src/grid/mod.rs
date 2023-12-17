//! Utilities for working with a two dimensional grid of values
//!
//! # Examples
//! ```
//! use aoc::grid::Grid;
//!
//! let mut grid = Grid::from_fn(10, 10, |_| 0);
//! grid.set((1, 1), 3);
//! assert_eq!(grid.get((0, 0)), Some(0));
//! assert_eq!(grid.get((1, 1)), Some(3));
//! ```

mod grid_pos;

use itertools::Itertools;
use std::fmt::Debug;

pub use self::grid_pos::GridPos;
pub use crate::pos;

/// A two dimensional grid of values
#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    /// panics if grid is empty
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Self {
            width,
            height,
            grid,
        }
    }

    /// Create a grid of the specified size and fill it with the default cell value
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc::grid::Grid;
    /// let grid: Grid<usize> = Grid::from_default(10, 10);
    /// assert_eq!(grid.get((0, 0)), Some(0));
    /// ```
    pub fn from_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let grid = (0..height)
            .map(|_| (0..height).map(|_| Default::default()).collect())
            .collect();
        Self {
            width,
            height,
            grid,
        }
    }

    /// Create a grid of the specified size and fill it by repeatingly calling the
    /// provided function.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc::grid::{Grid, GridPos};
    /// let grid = Grid::from_fn(10, 10, |GridPos(x, y)| x + y);
    /// assert_eq!(grid.get((3, 2)), Some(5));
    /// ```
    pub fn from_fn<F>(width: usize, height: usize, cell_fn: F) -> Self
    where
        F: Fn(GridPos) -> T,
    {
        let grid = (0..height)
            .map(|y| (0..height).map(|x| cell_fn(pos!(x, y))).collect())
            .collect();
        Self {
            width,
            height,
            grid,
        }
    }

    /// Get a grid value at a given position
    /// will panic if the position is out of bounds
    pub fn get_unchecked<C: Into<(isize, isize)>>(&self, coords: C) -> T
    where
        T: Copy,
    {
        let (x, y) = coords.into();
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.grid[y][x]
    }

    /// Get a mutable reference to the grid value at a given position
    /// will panic if the position is out of bounds
    pub fn get_unchecked_mut<C: Into<(isize, isize)>>(&mut self, coords: C) -> &mut T {
        let (x, y) = coords.into();
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        &mut self.grid[y][x]
    }

    /// Get the value at a given position
    /// Returns None when the provided position is out of bounds for the grid
    ///
    /// # Examples
    ///
    /// ```
    /// // Get a grid value using an x-y tuple
    /// use aoc::grid::Grid;
    /// let grid = Grid::from_default(10, 10);
    /// assert_eq!(grid.get((0, 0)), Some(0));
    /// ```
    ///
    /// ```
    /// // Get a grid value using a `GridPos`
    /// use aoc::{pos, grid::{Grid, GridPos}};
    /// let grid = Grid::from_default(10, 10);
    /// assert_eq!(grid.get(pos![0, 0]), Some(0));
    /// ```
    pub fn get<C: Into<(isize, isize)>>(&self, coords: C) -> Option<T>
    where
        T: Copy,
    {
        let (x, y) = coords.into();
        ((0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y))
            .then(|| self.get_unchecked((x, y)))
    }

    /// Set the value at a given position
    /// Returns None when the provided position is out of bounds for the grid
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc::grid::Grid;
    /// let mut grid = Grid::from_default(10, 10);
    /// grid.set((0, 0), 1);
    /// assert_eq!(grid.get((0, 0)), Some(1));
    /// ```
    pub fn set<C: Into<(isize, isize)>>(&mut self, coords: C, value: T) -> Result<(), String>
    where
        C: Debug + Clone,
    {
        if let Some(cell) = self.get_mut(coords.clone()) {
            *cell = value;
            Ok(())
        } else {
            Err(format!("Grid coords {:?} is out of bounds.", coords))
        }
    }

    /// Get a mutable reference to the value at a given grid position
    /// Returns None when the provided position is out of bounds for the grid
    pub fn get_mut<C: Into<(isize, isize)>>(&mut self, coords: C) -> Option<&mut T> {
        let (x, y) = coords.into();
        ((0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y))
            .then(|| self.get_unchecked_mut((x, y)))
    }

    /// Get an iterator over every cell value in row-col order
    pub fn cells(self) -> impl Iterator<Item = T> {
        self.grid.into_iter().flat_map(|row| row.into_iter())
    }

    /// Get a referencing iterator over every cell value in row-col order
    pub fn cells_iter(&self) -> impl Iterator<Item = &T> {
        self.grid.iter().flat_map(|row| row.iter())
    }

    /// Get an iterator to every grid position in row-col order
    pub fn positions(&self) -> impl Iterator<Item = GridPos> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| pos!(x, y))
    }

    /// Print the grid using the provided formatting method
    pub fn print_cells<F, O>(&self, fmt_fn: F)
    where
        O: std::fmt::Display,
        F: Fn(GridPos, &T) -> O,
    {
        println!("Grid(");
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", fmt_fn(pos!(x, y), &self.grid[y][x]));
            }
            println!();
        }
        println!(")")
    }
}

impl<T: Debug> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid(")?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?} ", self.grid[y][x])?;
            }
            writeln!(f)?;
        }
        writeln!(f, ")")
    }
}

impl<T: Debug> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.grid[y][x])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
