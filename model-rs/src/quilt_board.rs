use std::default::Default;

use piece::Piece;
use position::{Position, Dimension, Transformation};

/// The board on which the quilt is constructed.
///
/// Invariant:
///
///  - rows.len() == height
///
///  - for row in rows { row.len() == width }
pub struct QuiltBoard {
    dimension: Dimension,
    rows:      Box<[Box<[bool]>]>,
}

/// The ways in which a piece placement can fail.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PlacementError {
    OverhangsRight,
    OverhangsBottom,
    OverlapsPiece,
}

/// The width and height of the default quilt board.
pub const DEFAULT_DIMENSION: usize = 9;

impl QuiltBoard {
    /// Creates a new, empty board of the given dimensions.
    pub fn new(dimension: Dimension) -> Self {
        let mut rows = Vec::new();

        for _ in 0 .. dimension.height {
            rows.push(vec![false; dimension.width].into_boxed_slice());
        }

        QuiltBoard {
            dimension: dimension,
            rows:      rows.into_boxed_slice(),
        }
    }

    /// Returns the number of squares covered by pieces.
    pub fn positions_covered(&self) -> usize {
        let mut result = 0;

        for row in &*self.rows {
            for &b in &**row {
                if b { result += 1; }
            }
        }

        result
    }

    pub fn dimension(&self) -> Dimension {
        self.dimension
    }

    pub fn width(&self) -> usize {
        self.dimension.width
    }

    pub fn height(&self) -> usize {
        self.dimension.height
    }

    /// Is the given position in bounds for the quilt board?
    pub fn is_position_in_bounds(&self, position: Position) -> bool {
        self.dimension.contains(position)
    }

    pub fn is_position_covered(&self, position: Position) -> bool {
        self.is_position_in_bounds(position) &&
            self.rows[position.y][position.x]
    }

    /// Is there a `size`-by-`size` square covered?
    pub fn is_square_covered(&self, size: usize) -> bool {
        for y in 0 .. self.dimension.height - size + 1 {
            for x in 0 .. self.dimension.width - size + 1 {
                if self.is_square_covered_at(Position::new(x, y), size) {
                    return true;
                }
            }
        }

        false
    }

    /// Is there a `size`-by-`size` square covered with its upper left at the given position?
    fn is_square_covered_at(&self, position: Position, size: usize) -> bool {
        for y in position.y .. position.y + size {
            for x in position.x .. position.x + size {
                if ! self.is_position_covered(Position::new(x, y)) {
                    return false;
                }
            }
        }

        true
    }

    /// Can the given piece be added at the given position under the given transformation?
    ///
    /// Returns `Err` of a reason if it cannot.
    pub fn can_add_piece(&self, position: Position,
                         piece: &Piece,
                         transformation: Transformation)
                         -> Result<(), PlacementError>
    {
        for p in piece.positions(transformation) {
            let x = position.x + p.x;
            let y = position.y + p.y;

            if x >= self.dimension.width {
                return Err(PlacementError::OverhangsRight);
            } else if y >= self.dimension.height {
                return Err(PlacementError::OverhangsBottom);
            } else if self.rows[y][x] {
                return Err(PlacementError::OverlapsPiece);
            }
        }

        Ok(())
    }

    /// Adds the given piece at the specified position under the given transformation.
    pub fn add_piece(&mut self, position: Position, piece: &Piece, transformation: Transformation)
                     -> Result<(), PlacementError>
    {
        self.can_add_piece(position, piece, transformation)?;

        for p in piece.positions(transformation) {
            self.rows[position.y + p.y][position.x + p.x] = true;
        }

        Ok(())
    }
}

impl Default for QuiltBoard {
    fn default() -> Self {
        QuiltBoard::new(Dimension::new(DEFAULT_DIMENSION, DEFAULT_DIMENSION))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use piece::*;
    use position::{Transformation, Rotation, Flip};

    fn pos(x: usize, y: usize) -> Position {
        Position::new(x, y)
    }

    // ##
    //  #
    //  #
    fn a_piece() -> Piece {
        Piece::new(vec![pos(0, 0), pos(1, 0), pos(1, 1), pos(1, 2)], 0, 0)
    }

    #[test]
    fn place_a_piece_in_upper_left() {
        let mut board = QuiltBoard::default();

        assert_eq!(board.add_piece(pos(0, 0), &a_piece(), Transformation::identity()), Ok(()));

        assert_eq!(board.positions_covered(), 4);
        assert!(board.is_position_covered(pos(0, 0)));
        assert!(board.is_position_covered(pos(1, 0)));
        assert!(board.is_position_covered(pos(1, 1)));
        assert!(board.is_position_covered(pos(1, 2)));
        assert!(! board.is_position_covered(pos(0, 1)));
        assert!(! board.is_position_covered(pos(0, 2)));
        assert!(! board.is_position_covered(pos(2, 0)));
        assert!(! board.is_position_covered(pos(2, 1)));
    }

    #[test]
    fn place_four_pieces() {
        let mut board = QuiltBoard::default();

        // ------
        // --##--
        // ---#--
        // ---#--
        // ------
        // ------
        assert_eq!(board.add_piece(pos(2, 1), &a_piece(), Transformation::identity()), Ok(()));
        // ------
        // --##--
        // --##--
        // --##--
        // --##--
        // ------
        assert_eq!(board.add_piece(pos(2, 2), &a_piece(),
                                   Transformation::new(Rotation::Clockwise180, Flip::Identity)),
                   Ok(()));

        assert!(board.is_square_covered(2));
        assert!(!board.is_square_covered(3));

        // -------
        // --####-
        // --###--
        // --###--
        // --##---
        // -------
        assert_eq!(board.add_piece(pos(4, 1), &a_piece(),
                                   Transformation::new(Rotation::NoRotation, Flip::Horizontal)),
                   Ok(()));

        assert!(board.is_square_covered(3));
        assert!(!board.is_square_covered(4));

        // -------
        // --####-
        // --####-
        // --####-
        // --####-
        // -------
        assert_eq!(board.add_piece(pos(5, 2), &a_piece(),
                                   Transformation::new(Rotation::Clockwise180, Flip::Horizontal)),
                   Ok(()));

//        assert!(board.is_square_covered_at(pos(2, 1), 3));
//        assert!(!board.is_square_covered(5));
    }
}