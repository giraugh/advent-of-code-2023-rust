use derive_more::{Add, AddAssign, From, Into, Mul, MulAssign, Sub, SubAssign};

use crate::direction::Dir;

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
