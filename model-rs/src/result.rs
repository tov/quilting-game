
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
