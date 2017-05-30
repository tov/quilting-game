use std::collections::{vec_deque, VecDeque};

use rand;
use piece::Piece;

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

    pub fn take(&mut self, depth: usize) -> Result<Piece, String> {
        if depth > self.depth {
            Err("Can't take pieces that far in".to_owned())
        } else if depth >= self.piece_queue.len() {
            Err("There aren't that many pieces.".to_owned())
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

fn shuffle<R: rand::Rng, T>(rng: &mut R, vd: &mut VecDeque<T>) {
    use rand::distributions::{IndependentSample, Range};

    for i in (1 .. vd.len()).rev() {
        let range = Range::new(0, i);
        let j = range.ind_sample(rng);
        vd.swap(i, j);
    }
}