//! The board along which players’ pieces move, tracking time.

use std::collections::VecDeque;

use piece::Piece;

/// A single square on the time board.
pub struct Square {
    piece: Option<Piece>,
    collect: bool,
}

/// The time board.
pub struct TimeBoard {
    squares: VecDeque<Square>,
}