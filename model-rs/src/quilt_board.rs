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
}