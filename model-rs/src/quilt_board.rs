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
    pub fn squares_covered(&self) -> usize {
        let mut result = 0;

        for row in &*self.rows {
            for &b in &**row {
                if b { result += 1; }
            }
        }

        result
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
    pub fn is_square_covered_at(&self, position: Position, size: usize) -> bool {
        for y in position.y .. position.y + size {
            for x in position.x .. position.x + size {
                if x >= self.dimension.width || y >= self.dimension.height {
                    return false;
                }

                if ! self.rows[y][x] {
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