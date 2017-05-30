use std::cmp;
use std::slice;

/// A game piece
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    width: usize,
    height: usize,
    positions: Box<[(usize, usize)]>,
    cost: usize,
    distance: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    NoRotation,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flip {
    Identity,
    Horizontal,
}

/// Ways that a game piece can be positioned.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transformation {
    rotation: Rotation,
    flip:     Flip
}

impl Rotation {
    fn apply(self, width: usize, height: usize, x: usize, y: usize) -> (usize, usize) {
        use self::Rotation::*;

        match self {
            NoRotation      => (x, y),
            Clockwise90     => (height - y, x),
            Clockwise180    => (width - x, height - y),
            Clockwise270    => (y, width - x),
        }
    }
}

impl Flip {
    pub fn apply(self, width: usize, height: usize, x: usize, y: usize) -> (usize, usize) {
        use self::Flip::*;

        match self {
            Identity => (x, y),
            Horizontal  => (width - x, y),
        }
    }
}

impl Transformation {
    pub fn apply(self, width: usize, height: usize, x: usize, y: usize) -> (usize, usize) {
        let Transformation { rotation, flip } = self;
        let rotated = rotation.apply(width, height, x, y);
        flip.apply(width, height, rotated.0, rotated.1)
    }
}

#[derive(Debug, Clone)]
pub struct Positions<'a> {
    raw_positions: slice::Iter<'a, (usize, usize)>,
    transformation: Transformation,
    width: usize,
    height: usize,
}

impl Piece {
    pub fn width(&self, transformation: Transformation) -> usize {
        match transformation.rotation {
            Rotation::NoRotation | Rotation::Clockwise180 => self.width,
            Rotation::Clockwise90 | Rotation::Clockwise270 => self.height,
        }
    }

    pub fn height(&self, transformation: Transformation) -> usize {
        match transformation.rotation {
            Rotation::NoRotation | Rotation::Clockwise180 => self.height,
            Rotation::Clockwise90 | Rotation::Clockwise270 => self.width,
        }
    }

    pub fn cost(&self) -> usize {
        self.cost
    }

    pub fn distance(&self) -> usize {
        self.distance
    }

    pub fn positions(&self, transformation: Transformation) -> Positions {
        Positions {
            raw_positions: (&*self.positions).into_iter(),
            transformation: transformation,
            width: self.width,
            height: self.height,
        }
    }

    /// Does this `Piece` satisfy the invariant for `Piece`s?
    pub fn invariant(&self) -> bool {
        let (mut width, mut height) = (0, 0);

        for &(x, y) in &*self.positions {
            width = cmp::max(width, x + 1);
            height = cmp::max(height, y + 1);

            if (&*self.positions).into_iter().filter(|&&(xi, yi)| x == xi && y == yi).count() != 1 {
                return false;
            }
        }

        width == self.width && height == self.height
    }
}

impl<'a> Iterator for Positions<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_positions.next().map(|&(x, y)|
            self.transformation.apply(self.width, self.height, x, y))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_positions.size_hint()
    }
}

impl<'a> ExactSizeIterator for Positions<'a> {
    fn len(&self) -> usize {
        self.raw_positions.len()
    }
}
