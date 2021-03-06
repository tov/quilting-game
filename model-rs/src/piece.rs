//! Game pieces, representing quilt patches.

use std::{cmp, fmt, slice};

use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

use position::{Position, Dimension, Transformation};

/// A game piece
///
/// Invariant:
///
///  - The positions fit tightly within the dimension.
///
///  - The positions are sorted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Piece {
    /// The dimensions of the piece.
    #[serde(skip_serializing)]
    dimension: Dimension,
    /// The pieces positions.
    positions: Box<[Position]>,
    /// The cost of the piece.
    cost:      usize,
    /// The distance to move when taking the piece.
    distance:  usize,
    /// The value to collect when collecting, if holding the piece.
    collect:   usize,
}

impl Piece {
    /// Constructs a new piece from the given positions, cost, and move distance.
    pub fn new(mut positions: Vec<Position>, cost: usize, distance: usize, collect: usize) -> Self {
        positions.sort();
        positions.dedup();

        let dimension = compute_dimension(positions.iter());

        Piece {
            dimension: dimension,
            positions: positions.into_boxed_slice(),
            cost:      cost,
            distance:  distance,
            collect:   collect,
        }
    }

    /// A small square piece that is placed on the `TimeBoard`.
    pub fn single_position() -> Self {
        Self::new(vec![Position::new(0, 0)], 0, 0, 0)
    }

    /// Gets the dimension of this piece under the given transformation.
    pub fn dimension(&self, transformation: Transformation) -> Dimension {
        transformation.apply_dim(self.dimension)
    }

    /// Gets the width of this piece under the given transformation.
    pub fn width(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).width
    }

    /// Gets the height of this piece under the given transformation.
    pub fn height(&self, transformation: Transformation) -> usize {
        self.dimension(transformation).height
    }

    /// Gets the number of positions covered by the piece.
    pub fn size(&self) -> usize {
        self.positions.len()
    }

    /// Gets the cost of this piece.
    pub fn cost(&self) -> usize {
        self.cost
    }

    /// Gets the distance moved for this piece.
    pub fn distance(&self) -> usize {
        self.distance
    }

    /// Gets the value to collect when holding this piece.
    pub fn collect(&self) -> usize {
        self.collect
    }

    /// Gets an iterator over the positions of this piece under the given transformation.
    pub fn positions(&self, transformation: Transformation) -> Positions {
        Positions {
            raw_positions: (&*self.positions).into_iter(),
            raw_dimension: self.dimension,
            transformation: transformation,
        }
    }
}

impl<'de> Deserialize<'de> for Piece {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Positions, Cost, Distance, Collect, }

        struct PieceVisitor;

        impl<'de> Visitor<'de> for PieceVisitor {
            type Value = Piece;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Piece")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Piece, V::Error>
                where V: SeqAccess<'de>
            {
                let positions = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let cost = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let distance = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let collect = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(Piece::new(positions, cost, distance, collect))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Piece, V::Error>
                where V: MapAccess<'de>
            {
                let mut positions = None;
                let mut cost = None;
                let mut distance = None;
                let mut collect = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Positions => {
                            if positions.is_some() {
                                return Err(de::Error::duplicate_field("positions"));
                            }
                            positions = Some(map.next_value()?);
                        }
                        Field::Cost => {
                            if cost.is_some() {
                                return Err(de::Error::duplicate_field("cost"));
                            }
                            cost = Some(map.next_value()?);
                        }
                        Field::Distance => {
                            if distance.is_some() {
                                return Err(de::Error::duplicate_field("distance"));
                            }
                            distance = Some(map.next_value()?);
                        }
                        Field::Collect => {
                            if collect.is_some() {
                                return Err(de::Error::duplicate_field("collect"));
                            }
                            collect = Some(map.next_value()?);
                        }
                    }
                }

                let positions = positions.ok_or_else(|| de::Error::missing_field("positions"))?;
                let cost      = cost.ok_or_else(|| de::Error::missing_field("cost"))?;
                let distance  = distance.ok_or_else(|| de::Error::missing_field("distance"))?;
                let collect   = collect.ok_or_else(|| de::Error::missing_field("collect"))?;

                Ok(Piece::new(positions, cost, distance, collect))
            }
        }

        const FIELDS: &'static [&'static str] = &["positions", "cost", "distance", "collect"];
        deserializer.deserialize_struct("Piece", FIELDS, PieceVisitor)
    }
}

/// Computes the maximum dimension required to hold the given positions.
fn compute_dimension<'a, I>(positions: I) -> Dimension
    where I: Iterator<Item = &'a Position>
{
    let mut dimension = Dimension::new(0, 0);

    for &p in positions {
        dimension.width = cmp::max(dimension.width, p.x + 1);
        dimension.height = cmp::max(dimension.height, p.y + 1);
    }

    dimension
}

/// An iterator over the (transformed) positions of a `Piece`.
#[derive(Debug, Clone)]
pub struct Positions<'a> {
    raw_positions:  slice::Iter<'a, Position>,
    raw_dimension:  Dimension,
    transformation: Transformation,
}

impl<'a> Iterator for Positions<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw_positions.next().map(|&p|
            self.transformation.apply(self.raw_dimension, p))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw_positions.size_hint()
    }
}

impl<'a> ExactSizeIterator for Positions<'a> {
    fn len(&self) -> usize {
        self.raw_positions.len()
    }
}

/// Some sample pieces for testing with.
pub mod examples {
    use super::Piece;
    use position::Position;
    
    fn pos(x: usize, y: usize) -> Position {
        Position::new(x, y)
    }

    /// A piece.
    ///
    /// ```text
    /// ##
    ///  #
    ///  #
    /// ```
    pub fn piece0() -> Piece {
        Piece::new(vec![pos(0, 0), pos(1, 0), pos(1, 1), pos(1, 2)],
                   2, 1, 0)
    }

    /// A piece.
    ///
    /// ```text
    /// ##
    ///  #
    ///  #
    ///  ##
    /// ```
    pub fn piece1() -> Piece {
        Piece::new(vec![pos(0, 0), pos(1, 0), pos(1, 1), pos(1, 2), pos(1, 3), pos(2, 3)],
                   1, 2, 0)
    }

    /// A piece.
    ///
    /// ```text
    /// ##
    ///  ##
    ///  ##
    /// ```
    pub fn piece2() -> Piece {
        Piece::new(vec![pos(0, 0), pos(1, 0), pos(1, 1), pos(2, 1), pos(1, 2), pos(2, 2)],
                   8, 6, 3)
    }

    /// A piece.
    ///
    /// ```text
    ///  #
    /// ##
    /// ```
    pub fn piece3() -> Piece {
        Piece::new(vec![pos(1, 0), pos(0, 1), pos(1, 1)],
                   1, 3, 0)
    }

    /// A piece.
    ///
    /// ```text
    ///  #
    ///  #
    /// ###
    ///  #
    ///  #
    /// ```
    pub fn piece4() -> Piece {
        Piece::new(vec![pos(1, 0), pos(1, 1), pos(0, 2), pos(1, 2),
                        pos(2, 2), pos(1, 3), pos(1, 4)],
                   1, 4, 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use position::Rotation::*;
    use position::Flip::*;

    fn pos(x: usize, y: usize) -> Position {
        Position::new(x, y)
    }

    #[test]
    fn transform_width_height() {
        let piece = examples::piece0();
        let t1 = Transformation::new(NoRotation,   Identity);
        let t2 = Transformation::new(Clockwise90,  Identity);

        assert_eq!(piece.width(t1), 2);
        assert_eq!(piece.width(t2), 3);
        assert_eq!(piece.height(t1), 3);
        assert_eq!(piece.height(t2), 2);
    }

    #[test]
    fn positions_iterator_with_identity() {
        let piece = examples::piece0();
        // 01
        //  2
        //  3
        let mut positions = piece.positions(Transformation::new(NoRotation, Identity));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(1, 0)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(1, 2)));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn positions_iterator_with_0_h() {
        let piece = examples::piece0();
        // 10
        // 2
        // 3
        let mut positions = piece.positions(Transformation::new(NoRotation, Horizontal));
        assert_eq!(positions.next(), Some(pos(1, 0)));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(0, 2)));

    }

    #[test]
    fn positions_iterator_with_90_i() {
        let piece = examples::piece0();
        //   0
        // 321
        let mut positions = piece.positions(Transformation::new(Clockwise90, Identity));
        assert_eq!(positions.next(), Some(pos(2, 0)));
        assert_eq!(positions.next(), Some(pos(2, 1)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn positions_iterator_with_90_h() {
        let piece = examples::piece0();
        // 0
        // 123
        let mut positions = piece.positions(Transformation::new(Clockwise90, Horizontal));
        assert_eq!(positions.next(), Some(pos(0, 0)));
        assert_eq!(positions.next(), Some(pos(0, 1)));
        assert_eq!(positions.next(), Some(pos(1, 1)));
        assert_eq!(positions.next(), Some(pos(2, 1)));
        assert_eq!(positions.next(), None);
    }
}
