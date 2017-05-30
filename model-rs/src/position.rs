use std::default::Default;

/// A position on the board or in a piece.
///
/// Origin is in the upper left.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    /// Creates a new position with the given x and y coordinates.
    pub fn new(x: usize, y: usize) -> Self {
        Position {
            x: x,
            y: y,
        }
    }

    /// Translates a position relative to another.
    ///
    /// (Like vector addition.)
    pub fn translate(self, other: Position) -> Self {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// The dimensions of a board or piece.
///
/// Origin is in the upper left.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dimension {
    pub width: usize,
    pub height: usize,
}

impl Dimension {
    /// Creates a new `Dimension` with the given width and height.
    pub fn new(width: usize, height: usize) -> Self {
        Dimension {
            width: width,
            height: height,
        }
    }

    /// Is the given `Position` within the given `Dimension`?
    pub fn contains(self, p: Position) -> bool {
        p.x < self.width && p.y < self.height
    }

    /// Transposes (swaps) the width and height.
    pub fn transpose(self) -> Self {
        Dimension {
            width: self.height,
            height: self.width,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    NoRotation,
    Clockwise90,
    Clockwise180,
    Clockwise270,
}

impl Rotation {
    /// Applies this rotation to the given dimension.
    pub fn apply_dim(self, d: Dimension) -> Dimension {
        if self.is_even() {d} else {d.transpose()}
    }

    /// Applies this rotation to the given position relative to the given dimension.
    pub fn apply(self, d: Dimension, p: Position) -> Position {
        use self::Rotation::*;

        match self {
            NoRotation   => Position::new(p.x, p.y),
            Clockwise90  => Position::new(d.height - p.y - 1, p.x),
            Clockwise180 => Position::new(d.width - p.x - 1, d.height - p.y - 1),
            Clockwise270 => Position::new(p.y, d.width - p.x - 1),
        }
    }

    fn is_even(self) -> bool {
        self == Rotation::NoRotation || self == Rotation::Clockwise180
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flip {
    Identity,
    Horizontal,
}

impl Flip {
    /// Applies this flip transformation to the given position within the given dimension.
    pub fn apply(self, d: Dimension, p: Position) -> Position {
        use self::Flip::*;

        match self {
            Identity    => Position::new(p.x, p.y),
            Horizontal  => Position::new(d.width - p.x - 1, p.y),
        }
    }
}

/// Ways that a game piece can be positioned.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Transformation {
    rotation: Rotation,
    flip:     Flip
}

impl Transformation {
    /// Creates a new transformation as the composition of a rotation (first) and a flip (second).
    pub fn new(rotation: Rotation, flip: Flip) -> Self {
        Transformation {
            rotation: rotation,
            flip: flip,
        }
    }

    /// The identity tranformation.
    pub fn identity() -> Self {
        Transformation::new(Rotation::NoRotation, Flip::Identity)
    }

    /// Applies this transformation to a dimension.
    pub fn apply_dim(self, d: Dimension) -> Dimension {
        self.rotation.apply_dim(d)
    }

    /// Applies this tranformation to a position within the given dimension.
    pub fn apply(self, d: Dimension, p: Position) -> Position {
        let Transformation { rotation, flip } = self;
        let p = rotation.apply(d, p);
        let d = rotation.apply_dim(d);
        flip.apply(d, p)
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Rotation::*;
    use super::Flip::*;

    fn pos(x: usize, y: usize) -> Position {
        Position::new(x, y)
    }

    #[test]
    fn transform_upper_left() {
        let d = Dimension::new(6, 4);
        let p = Position::new(0, 0);
        let check = |r, f, x, y| assert_eq!(Transformation::new(r, f).apply(d, p), pos(x, y));

        check(NoRotation,   Identity,   0, 0);
        check(Clockwise90,  Identity,   3, 0);
        check(Clockwise180, Identity,   5, 3);
        check(Clockwise270, Identity,   0, 5);
        check(NoRotation,   Horizontal, 5, 0);
        check(Clockwise90,  Horizontal, 0, 0);
        check(Clockwise180, Horizontal, 0, 3);
        check(Clockwise270, Horizontal, 3, 5);
    }

    #[test]
    fn transform_upper_right() {
        let d = Dimension::new(8, 6);
        let p = Position::new(7, 0);
        let check = |r, f, x, y| assert_eq!(Transformation::new(r, f).apply(d, p), pos(x, y));

        check(NoRotation,   Identity,   7, 0);
        check(Clockwise90,  Identity,   5, 7);
        check(Clockwise180, Identity,   0, 5);
        check(Clockwise270, Identity,   0, 0);
        check(NoRotation,   Horizontal, 0, 0);
        check(Clockwise90,  Horizontal, 0, 7);
        check(Clockwise180, Horizontal, 7, 5);
        check(Clockwise270, Horizontal, 5, 0);
    }

    #[test]
    fn transform_2_1() {
        let d = Dimension::new(6, 4);
        let p = Position::new(2, 1);
        let check = |r, f, x, y| assert_eq!(Transformation::new(r, f).apply(d, p), pos(x, y));

        check(NoRotation,   Identity,   2, 1);
        check(Clockwise90,  Identity,   2, 2);
        check(Clockwise180, Identity,   3, 2);
        check(Clockwise270, Identity,   1, 3);
        check(NoRotation,   Horizontal, 3, 1);
        check(Clockwise90,  Horizontal, 1, 2);
        check(Clockwise180, Horizontal, 2, 2);
        check(Clockwise270, Horizontal, 2, 3);
    }

    #[test]
    fn transform_width_height() {
        let d = Dimension::new(2, 3);
        let t1 = Transformation::new(NoRotation,   Identity);
        let t2 = Transformation::new(Clockwise90,  Identity);

        let d1 = t1.apply_dim(d);
        let d2 = t2.apply_dim(d);

        assert_eq!(d1, d);
        assert_eq!(d2, Dimension::new(3, 2));
    }
}
