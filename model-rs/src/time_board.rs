//! The board along which players’ pieces move, tracking time.

use piece::Piece;

/// A single square on the time board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Square {
    piece:   Option<Piece>,
    collect: bool,
}

/// The time board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeBoard {
    squares: Box<[Square]>,
}