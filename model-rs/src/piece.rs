use std::cmp;
use std::slice;

use position::{Position, Dimension, Transformation};

/// A game piece
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    dimension: Dimension,
    positions: Box<[Position]>,
    cost:      usize,
    distance:  usize,
}

impl Piece {
    /// Does this `Piece` satisfy the invariant for `Piece`s?
    ///
    /// In particular:
    ///
    ///  - Do the positions fit tightly within the dimension?
    ///
    ///  - Are the positions sorted?
    pub fn invariant(&self) -> bool {
        let mut dimension = Dimension::new(0, 0);
        let mut previous  = None;

        for &p in &*self.positions {
            dimension.width = cmp::max(dimension.width, p.x + 1);
            dimension.height = cmp::max(dimension.height, p.y + 1);

            if let Some(previous) = previous {
                if previous >= p {
                    return false;
                }
            }

            previous = Some(p);
        }

        dimension == self.dimension
    }

    pub fn dimension(&self, transformation: Transformation) -> Dimension {
        transformation.apply_dim(self.dimension)
    }

    pub fn width(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).width
    }

    pub fn height(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).height
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
            raw_dimension: self.dimension,
            transformation: transformation,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Positions<'a> {
    raw_positions:  slice::Iter<'a, Position>,
    raw_dimension:  Dimension,
    transformation: Transformation,
}

impl<'a> Iterator for Positions<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_positions.next().map(|&p|
            self.transformation.apply(self.raw_dimension, p))
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
    use position::Rotation::*;
    use position::Flip::*;

    fn pos(x: usize, y: usize) -> Position {
        Position::new(x, y)
    }

    // ##
    //  #
    //  #
    fn a_piece() -> Piece {
        Piece {
            dimension: Dimension::new(2, 3),
            positions: vec![pos(0, 0), pos(1, 0), pos(1, 1), pos(1, 2)].into_boxed_slice(),
            cost: 0,
            distance: 0,
        }
    }

    #[test]
    fn transform_width_height() {
        let piece = a_piece();
        let t1 = Transformation::new(NoRotation,   Identity);
        let t2 = Transformation::new(Clockwise90,  Identity);

        assert_eq!(piece.width(t1), 2);
        assert_eq!(piece.width(t2), 3);
        assert_eq!(piece.height(t1), 3);
        assert_eq!(piece.height(t2), 2);
    }

    #[test]
    fn positions_iterator_with_identity() {
        let piece = a_piece();
        let mut positions = piece.positions(Transformation::new(NoRotation, Identity));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(1, 0)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(1, 2)));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn positions_iterator_with_0_h() {
        let piece = a_piece();
        let mut positions = piece.positions(Transformation::new(NoRotation, Horizontal));
        assert_eq!(positions.next(), Some(pos(1, 0)));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(0, 2)));

    }

    #[test]
    fn positions_iterator_with_90_i() {
        let piece = a_piece();
        let mut positions = piece.positions(Transformation::new(Clockwise90, Identity));
        assert_eq!(positions.next(), Some(pos(2, 0)));
        assert_eq!(positions.next(), Some(pos(2, 1)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn positions_iterator_with_90_h() {
        let piece = a_piece();
        let mut positions = piece.positions(Transformation::new(Clockwise90, Horizontal));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(2, 1)));
        assert_eq!(positions.next(), None);
    }
}
