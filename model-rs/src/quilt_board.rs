use piece::Piece;

pub struct QuiltBoard {
    width:  usize,
    height: usize,
    rows:   Box<[Box<[bool]>]>,
}

impl QuiltBoard {
    pub fn invariant(&self) -> bool {
        for row in &*self.rows {
            if row.len() != self.width {
                return false;
            }
        }

        self.rows.len() == self.height
    }

    pub fn can_add_piece(&self, piece: Piece) -> bool {
        false
    }

    pub fn add_piece(x: usize, y: usize, piece: Piece) -> Result<(), Piece> {
        Err(piece)
    }
}