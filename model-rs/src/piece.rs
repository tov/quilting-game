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
pub enum Transformation {
    FlipHorizontal {
        width: usize,
    },
    Rotation {
        clockwise90s: usize,
        width: usize,
        height: usize,
    }
}

impl Transformation {
    pub fn apply(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Transformation::FlipHorizontal { width } => (width - x, y),
            Transformation::Rotation { clockwise90s, width, height } => {
                match clockwise90s % 4 {
                    0 => (x, y),
                    1 => (height - y, x),
                    2 => (width - x, height - y),
                    3 => (y, width - x),
                    _ => unreachable!(),
                }
            }
        }

    }
}

pub struct Positions<'a> {
    raw_positions: slice::Iter<'a, (usize, usize)>,
    transformation: Transformation,
}

impl Piece {
    pub fn width(&self, transformation: Transformation) -> usize {
        transformation.apply(self.width, self.height).0
    }

    pub fn height(&self, transformation: Transformation) -> usize {
        transformation.apply(self.width, self.height).1
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
        self.raw_positions.next().map(|&(x, y)| self.transformation.apply(x, y))
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
