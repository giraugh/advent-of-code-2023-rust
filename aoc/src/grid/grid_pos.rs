use derive_more::{Add, AddAssign, From, Into, Mul, MulAssign, Sub, SubAssign};

use crate::direction::Dir;

use super::Grid;

/// A signed position or offset into a grid
#[derive(
    Clone, Copy, PartialEq, Eq, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Into, From, Hash,
)]
pub struct GridPos(pub isize, pub isize);

/// Helper for constructing a position
#[macro_export]
macro_rules! pos {
    ($x: expr, $y: expr) => {
        GridPos($x as isize, $y as isize)
    };
}

impl GridPos {
    /// Get cartesian neighbours of this position
    /// (not guaranteed to be in bounds)
    pub fn neighbours(&self) -> impl Iterator<Item = Self> {
        let (x, y) = (self.0, self.1);
        [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .map(|(x, y)| Self(x, y))
    }

    /// Whether this position is in bounds for a given grid
    pub fn in_grid<T>(&self, grid: &Grid<T>) -> bool {
        (0..grid.width as isize).contains(&self.0) && (0..grid.height as isize).contains(&self.1)
    }
}

impl std::fmt::Debug for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos({}, {})", self.0, self.1)
    }
}

impl From<Dir> for GridPos {
    fn from(value: Dir) -> Self {
        match value {
            Dir::North => pos!(0, -1),
            Dir::South => pos!(0, 1),
            Dir::West => pos!(-1, 0),
            Dir::East => pos!(1, 0),
        }
    }
}

impl TryFrom<GridPos> for Dir {
    type Error = String;

    fn try_from(value: GridPos) -> Result<Self, Self::Error> {
        use Dir::*;

        let (x_sign, y_sign) = (value.0.signum(), value.1.signum());
        match (x_sign, y_sign) {
            (1, 0) => Ok(East),
            (-1, 0) => Ok(West),
            (0, 1) => Ok(South),
            (0, -1) => Ok(North),
            p => Err(format!("Dir cannot represent position {p:?}")),
        }
    }
}
