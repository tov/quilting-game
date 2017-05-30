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
            Clockwise90     => (height - y - 1, x),
            Clockwise180    => (width - x - 1, height - y - 1),
            Clockwise270    => (y, width - x - 1),
        }
    }

    fn is_even(self) -> bool {
        self == Rotation::NoRotation || self == Rotation::Clockwise180
    }
}

impl Flip {
    pub fn apply(self, width: usize, height: usize, x: usize, y: usize) -> (usize, usize) {
        use self::Flip::*;

        match self {
            Identity    => (x, y),
            Horizontal  => (width - x - 1, y),
        }
    }
}

impl Transformation {
    pub fn new(rotation: Rotation, flip: Flip) -> Self {
        Transformation {
            rotation: rotation,
            flip: flip,
        }
    }

    pub fn apply(self, width: usize, height: usize, x: usize, y: usize) -> (usize, usize) {
        let Transformation { rotation, flip } = self;
        let rotated = rotation.apply(width, height, x, y);
        if rotation.is_even() {
            flip.apply(width, height, rotated.0, rotated.1)
        } else {
            flip.apply(height, width, rotated.0, rotated.1)
        }
    }
}

impl Piece {
    pub fn width(&self, transformation: Transformation) -> usize {
        if transformation.rotation.is_even() {
            self.width
        } else {
            self.height
        }
    }

    pub fn height(&self, transformation: Transformation) -> usize {
        if transformation.rotation.is_even() {
            self.height
        } else {
            self.width
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

#[derive(Debug, Clone)]
pub struct Positions<'a> {
    raw_positions: slice::Iter<'a, (usize, usize)>,
    transformation: Transformation,
    width: usize,
    height: usize,
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

#[cfg(test)]
mod test {
    use super::*;
    use super::Rotation::*;
    use super::Flip::*;

    #[test]
    fn transform_upper_left() {
        assert_eq!(Transformation::new(NoRotation,   Identity).apply(6, 4, 0, 0), (0, 0));
        assert_eq!(Transformation::new(Clockwise90,  Identity).apply(6, 4, 0, 0), (3, 0));
        assert_eq!(Transformation::new(Clockwise180, Identity).apply(6, 4, 0, 0), (5, 3));
        assert_eq!(Transformation::new(Clockwise270, Identity).apply(6, 4, 0, 0), (0, 5));
        assert_eq!(Transformation::new(NoRotation,   Horizontal).apply(6, 4, 0, 0), (5, 0));
        assert_eq!(Transformation::new(Clockwise90,  Horizontal).apply(6, 4, 0, 0), (0, 0));
        assert_eq!(Transformation::new(Clockwise180, Horizontal).apply(6, 4, 0, 0), (0, 3));
        assert_eq!(Transformation::new(Clockwise270, Horizontal).apply(6, 4, 0, 0), (3, 5));
    }

    #[test]
    fn transform_upper_right() {
        assert_eq!(Transformation::new(NoRotation,   Identity).apply(8, 6, 7, 0), (7, 0));
        assert_eq!(Transformation::new(Clockwise90,  Identity).apply(8, 6, 7, 0), (5, 7));
        assert_eq!(Transformation::new(Clockwise180, Identity).apply(8, 6, 7, 0), (0, 5));
        assert_eq!(Transformation::new(Clockwise270, Identity).apply(8, 6, 7, 0), (0, 0));
        assert_eq!(Transformation::new(NoRotation,   Horizontal).apply(8, 6, 7, 0), (0, 0));
        assert_eq!(Transformation::new(Clockwise90,  Horizontal).apply(8, 6, 7, 0), (0, 7));
        assert_eq!(Transformation::new(Clockwise180, Horizontal).apply(8, 6, 7, 0), (7, 5));
        assert_eq!(Transformation::new(Clockwise270, Horizontal).apply(8, 6, 7, 0), (5, 0));
    }

    #[test]
    fn transform_2_1() {
        assert_eq!(Transformation::new(NoRotation,   Identity).apply(6, 4, 2, 1), (2, 1));
        assert_eq!(Transformation::new(Clockwise90,  Identity).apply(6, 4, 2, 1), (2, 2));
        assert_eq!(Transformation::new(Clockwise180, Identity).apply(6, 4, 2, 1), (3, 2));
        assert_eq!(Transformation::new(Clockwise270, Identity).apply(6, 4, 2, 1), (1, 3));
        assert_eq!(Transformation::new(NoRotation,   Horizontal).apply(6, 4, 2, 1), (3, 1));
        assert_eq!(Transformation::new(Clockwise90,  Horizontal).apply(6, 4, 2, 1), (1, 2));
        assert_eq!(Transformation::new(Clockwise180, Horizontal).apply(6, 4, 2, 1), (2, 2));
        assert_eq!(Transformation::new(Clockwise270, Horizontal).apply(6, 4, 2, 1), (2, 3));
    }
}
