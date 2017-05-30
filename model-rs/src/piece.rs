use std::cmp;
use std::slice;

use position::{Position, Dimension, Transformation};

/// A game piece
///
/// Invariant:
///
///  - The positions fit tightly within the dimension.
///
///  - The positions are sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    dimension: Dimension,
    positions: Box<[Position]>,
    cost:      usize,
    distance:  usize,
}

impl Piece {
    /// Constructs a new piece from the given positions, cost, and move distance.
    pub fn new<I>(positions: I, cost: usize, distance: usize) -> Self
        where I: IntoIterator<Item = Position>
    {
        let mut positions: Vec<_> = positions.into_iter().collect();
        positions.sort();
        positions.dedup();

        let dimension = compute_dimension(positions.iter());

        Piece {
            dimension: dimension,
            positions: positions.into_boxed_slice(),
            cost:      cost,
            distance:  distance,
        }
    }

    /// Gets the dimension of this piece under the given transformation.
    pub fn dimension(&self, transformation: Transformation) -> Dimension {
        transformation.apply_dim(self.dimension)
    }

    /// Gets the width of this piece under the given transformation.
    pub fn width(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).width
    }

    /// Gets the height of this piece under the given transformation.
    pub fn height(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).height
    }

    /// Gets the cost of this piece.
    pub fn cost(&self) -> usize {
        self.cost
    }

    /// Gets the distance moved for this piece.
    pub fn distance(&self) -> usize {
        self.distance
    }

    /// Gets an iterator over the positions of this piece under the given transformation.
    pub fn positions(&self, transformation: Transformation) -> Positions {
        Positions {
            raw_positions: (&*self.positions).into_iter(),
            raw_dimension: self.dimension,
            transformation: transformation,
        }
    }
}

/// Computes the maximum dimension required to hold the given positions.
fn compute_dimension<'a, I>(positions: I) -> Dimension
    where I: Iterator<Item = &'a Position>
{
    let mut dimension = Dimension::new(0, 0);

    for &p in positions {
        dimension.width = cmp::max(dimension.width, p.x + 1);
        dimension.height = cmp::max(dimension.height, p.y + 1);
    }

    dimension
}

/// An iterator over the (transformed) positions of a `Piece`.
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

    // 01
    //  2
    //  3
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
        // 01
        //  2
        //  3
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
        // 10
        // 2
        // 3
        let mut positions = piece.positions(Transformation::new(NoRotation, Horizontal));
        assert_eq!(positions.next(), Some(pos(1, 0)));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(0, 2)));

    }

    #[test]
    fn positions_iterator_with_90_i() {
        let piece = a_piece();
        //   0
        // 321
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
        // 0
        // 123
        let mut positions = piece.positions(Transformation::new(Clockwise90, Horizontal));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(2, 1)));
        assert_eq!(positions.next(), None);
    }
}
