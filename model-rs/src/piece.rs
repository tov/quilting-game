use std::cmp;

/// A game piece
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    width: usize,
    height: usize,
    positions: Box<[(usize, usize)]>,
    cost: usize,
    distance: usize,
}

impl Piece {
    /// Does this `Piece` satisfy the invariant for `Piece`s?
    pub fn invariant(&self) -> bool {
        let (mut width, mut height) = (0, 0);

        for &(x, y) in &*self.positions {
            width = cmp::max(width, x + 1);
            height = cmp::max(height, y + 1);

            if (&*self.positions).into_iter().filter(|&&(xi, yi)| x == xi && y == yi).count() != 1 {
                return false;
            }
        }

        width == self.width && height == self.height
    }
}
