//! Quilting game results and errors.

use std::{error, fmt};

/// The Quilting game result type.
pub type QResult<T> = Result<T, PlayerError>;

/// Errors that can be attributed to players.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerError {
    /// The piece cannot be placed because it overhangs the right edge of the board.
    PlacementOverhangsRight,
    /// The piece cannot be placed because it overhangs the bottom edge of the board.
    PlacementOverhangsBottom,
    /// The piece cannot be placed because it overlaps another piece.
    PlacementOverlapsPiece,
    /// Cannot take pieces from that deep in the piece queue.
    TakeOverDepth,
    /// The piece queue does not have that many pieces.
    OutOfPieces,
}

impl fmt::Display for PlayerError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(error::Error::description(self))
    }
}

impl error::Error for PlayerError {
    fn description(&self) -> &str {
        use self::PlayerError::*;

        match *self {
            PlacementOverhangsRight => "Piece placement overhangs right edge of quilt board",
            PlacementOverhangsBottom => "Piece placement overhangs bottom edge of quilt board",
            PlacementOverlapsPiece => "Piece placement overlaps another piece",
            TakeOverDepth => "Cannot take pieces from that deep in the queue",
            OutOfPieces => "The queue does not have that many pieces",
        }
    }
}
