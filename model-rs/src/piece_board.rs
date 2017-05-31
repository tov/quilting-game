use std::collections::{vec_deque, VecDeque};

use rand;
use piece::Piece;

use result::{QResult, PlayerError};

/// The queue of pieces to be taken.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PieceBoard {
    piece_queue: VecDeque<Piece>,
    depth: usize,
}

#[derive(Debug, Clone)]
pub struct Pieces<'a>(vec_deque::Iter<'a, Piece>);

impl PieceBoard {
    pub fn new<I>(piece_queue: I, depth: usize) -> Self
        where I: IntoIterator<Item = Piece>
    {
        PieceBoard {
            piece_queue: piece_queue.into_iter().collect(),
            depth: depth
        }
    }

    pub fn random<I>(piece_queue: I, depth: usize) -> Self
        where I: IntoIterator<Item = Piece>
    {
        let mut result = Self::new(piece_queue, depth);
        shuffle(&mut rand::thread_rng(), &mut result.piece_queue);
        result
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn pieces(&self) -> Pieces {
        Pieces(self.piece_queue.iter())
    }

    pub fn take(&mut self, depth: usize) -> QResult<Piece> {
        if depth > self.depth {
            Err(PlayerError::TakeOverDepth)
        } else if depth >= self.piece_queue.len() {
            Err(PlayerError::OutOfPieces)
        } else {
            let mut stack = Vec::new();
            for _ in 0..depth {
                stack.push(self.piece_queue.pop_front().unwrap());
            }
            let result = self.piece_queue.pop_front().unwrap();
            for piece in stack.into_iter().rev() {
                self.piece_queue.push_front(piece)
            }
            Ok(result)
        }
    }
}

impl<'a> Iterator for Pieces<'a> {
    type Item = &'a Piece;

    fn next(&mut self) -> Option<&'a Piece> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Pieces<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> DoubleEndedIterator for Pieces<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

/// [Fisher-Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle).
fn shuffle<R: rand::Rng, T>(rng: &mut R, vd: &mut VecDeque<T>) {
    use rand::distributions::{IndependentSample, Range};

    for i in (1 .. vd.len()).rev() {
        let range = Range::new(0, i);
        let j = range.ind_sample(rng);
        vd.swap(i, j);
    }
}