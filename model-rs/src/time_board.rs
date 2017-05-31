//! The board along which players’ pieces move, tracking time.

use std::default::Default;

use serde_json;

use piece::Piece;
use player::PlayOrder;

const TIME_BOARD_JSON: &'static [u8] = include_bytes!("../data/time_board.json");

/// The time board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeBoard {
    squares: Box<[Square]>,
}

/// A single square on the time board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Square {
    /// The piece to get if landing on or passing this square for the first time.
    #[serde(default)]
    piece:   Option<Piece>,
    /// Whether to collect money when landing on or passing this square.
    #[serde(default)]
    collect: bool,
    /// The players currently sitting on this square, in their order of play.
    #[serde(skip)]
    #[serde(default = "PlayOrder::empty")]
    players: PlayOrder,
}

impl TimeBoard {
    /// Creates a new time board with the default configuration and the given play order on the
    /// 0th square.
    pub fn new(play_order: PlayOrder) -> Self {
        let mut squares: Box<[Square]> = serde_json::from_slice(TIME_BOARD_JSON).unwrap();

        squares[0].players = play_order;

        TimeBoard {
            squares: squares,
        }
    }
}

impl Default for TimeBoard {
    fn default() -> Self {
        Self::new(PlayOrder::default())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn make_default_time_board() {
        let _time_board = TimeBoard::default();
    }
}
