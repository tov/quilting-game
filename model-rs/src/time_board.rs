use std::collections::VecDeque;

use piece::Piece;

pub struct Square {
    piece: Option<Piece>,
    collect: bool,
}

pub struct TimeBoard {
    squares: VecDeque<Square>,
}