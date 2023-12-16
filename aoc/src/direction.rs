#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrthDir {
    Horizontal,
    Vertical,
}

impl Dir {
    pub fn orthogonal(&self) -> OrthDir {
        use Dir::*;
        use OrthDir::*;
        match self {
            North => Vertical,
            South => Vertical,
            East => Horizontal,
            West => Horizontal,
        }
    }

    pub fn opposite(&self) -> Self {
        use Dir::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    pub fn turn_right(&self) -> Self {
        use Dir::*;
        match self {
            North => East,
            South => West,
            East => South,
            West => North,
        }
    }

    pub fn turn_left(&self) -> Self {
        use Dir::*;
        match self {
            North => West,
            South => East,
            East => North,
            West => South,
        }
    }
}
