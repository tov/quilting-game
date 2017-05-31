//! The board along which players’ pieces move, tracking time.

use std::cmp;
use std::default::Default;

use serde_json;

use piece::Piece;
use player::{Player, PlayOrder, Players};

const TIME_BOARD_JSON: &'static [u8] = include_bytes!("../data/time_board.json");

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
    #[serde(default = "PlayOrder::empty")]
    players: PlayOrder,
}

impl Square {
    /// Views the `Piece` to be taken when landing on or passing this square, if any.
    pub fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }

    /// Gets whether to collect money when landing on or passing this square.
    pub fn collect(&self) -> bool {
        self.collect
    }

    /// Gets whether this square has a player on it.
    pub fn has_player(&self) -> bool {
        ! self.players.is_empty()
    }

    /// Gets the sequence of players waiting on this square.
    pub fn players(&self) -> Players {
        self.players.players()
    }
}

/// The result of moving along the `PieceBoard`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveResult {
    /// Any pieces acquired from the move.
    pub pieces:   Vec<Piece>,
    /// The number of collections resulting from the move.
    pub collects: usize,
    /// The actual distance moved.
    pub distance: usize,
}

/// The time board.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct TimeBoard {
    squares: Box<[Square]>,
}

impl TimeBoard {
    /// Creates a new time board with the default configuration and the given play order on the
    /// 0th square.
    pub fn new(play_order: PlayOrder) -> Self {
        Self::from_slice(play_order, TIME_BOARD_JSON).unwrap()
    }

    /// Deserializes a new time board from a JSON slice.
    pub fn from_slice(play_order: PlayOrder, json: &[u8]) -> serde_json::Result<Self> {
        assert!(play_order.len() >= 2, "Cannot play with fewer than two players");

        let mut squares: Box<[Square]> = serde_json::from_slice(json)?;
        squares[0].players = play_order;

        Ok(TimeBoard {
            squares: squares,
        })
    }

    /// Views the squares of the time board.
    pub fn squares(&self) -> &[Square] {
        &*self.squares
    }

    /// Gets the index of the last square.
    pub fn index_of_last_square(&self) -> usize {
        self.squares().len() - 1
    }

    /// Returns the board position of the player whose turn it is.
    ///
    /// If the game is over, this returns the index of the last square.
    pub fn index_of_current_player(&self) -> usize {
        for (i, square) in self.squares().into_iter().enumerate() {
            if square.has_player() {
                return i;
            }
        }

        unreachable!("There must be at least one player");
    }

    /// Is the current game over?
    ///
    /// This is true when the current player is in the last square.
    pub fn is_game_over(&self) -> bool {
        self.index_of_current_player() == self.index_of_last_square()
    }

    /// Gets a reference to the player whose turn it is.
    ///
    /// Returns `None` if the game is over.
    pub fn current_player(&self) -> Option<&Player> {
        let i = self.index_of_current_player();

        if i == self.index_of_last_square() {
            None
        } else {
            self.squares()[i].players().next()
        }
    }

    /// Returns the board position of the player whose turn will be next.
    ///
    /// This is the position that the current player must pass to complete their turn.
    pub fn index_of_next_player(&self) -> usize {
        let first_index = self.index_of_current_player();

        if self.squares[first_index].players.len() > 1 {
            return first_index;
        }

        for i in first_index + 1 .. self.squares().len() {
            if self.squares()[i].has_player() {
                return i;
            }
        }

        unreachable!("There must be at least two players");
    }

    pub fn move_player(&mut self, distance: usize) -> MoveResult {
        assert!(distance > 0, "Cannot move distance of 0");
        assert!(! self.is_game_over(), "Cannot move if game is over");

        let start  = self.index_of_current_player();
        let stop   = cmp::min(start + distance, self.index_of_last_square());

        let player = self.squares[start].players.pop().unwrap();
        self.squares[stop].players.push(player);

        let mut result = MoveResult {
            pieces:   Vec::new(),
            collects: 0,
            distance: stop - start,
        };

        for square in &mut self.squares[start + 1 .. stop + 1] {
            if let Some(piece) = square.piece.take() {
                result.pieces.push(piece)
            }

            if square.collect() {
                result.collects += 1;
            }
        }

        result
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

    static TEST_BOARD: &'static [u8] =
        br#"[
          {},
          {},
          {},
          {},
          {},
          {
            "collect": true
          },
          {},
          {
            "collect": true
          },
          {},
          {
            "collect": true
          },
          {
            "piece": {
              "positions": [{"x": 0, "y": 0}],
              "cost": 0,
              "distance": 0,
              "collect": 0
            }
          },
          {},
          {},
          {},
          {
            "collect": true
          }
        ]"#;

    #[test]
    fn make_default_time_board() {
        let time_board = TimeBoard::default();
        assert_eq!(time_board.index_of_current_player(), 0);
        assert_eq!(time_board.index_of_next_player(), 0);
    }

    #[test]
    fn move_pieces() {
        let play_order = PlayOrder::new(2);

        // [01][][][][][C][][C][][C][P][][][][C]
        let mut time_board = TimeBoard::from_slice(play_order.clone(), TEST_BOARD).unwrap();

        assert_eq!(time_board.current_player(), play_order.players().nth(0));
        assert_eq!(time_board.index_of_current_player(), 0);
        assert_eq!(time_board.index_of_next_player(), 0);

        // [1][][0][][][C][][C][][C][P][][][][C]
        let mr = time_board.move_player(2);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 0);
        assert_eq!(mr.distance, 2);

        assert_eq!(time_board.current_player(), play_order.players().nth(1));
        assert_eq!(time_board.index_of_current_player(), 0);
        assert_eq!(time_board.index_of_next_player(), 2);

        // [][][10][][][C][][C][][C][P][][][][C]
        let mr = time_board.move_player(2);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 0);
        assert_eq!(mr.distance, 2);

        assert_eq!(time_board.current_player(), play_order.players().nth(1));
        assert_eq!(time_board.index_of_current_player(), 2);
        assert_eq!(time_board.index_of_next_player(), 2);

        // [][][0][][][C1][][C][][C][P][][][][C]
        let mr = time_board.move_player(3);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 1);
        assert_eq!(mr.distance, 3);

        assert_eq!(time_board.current_player(), play_order.players().nth(0));
        assert_eq!(time_board.index_of_current_player(), 2);
        assert_eq!(time_board.index_of_next_player(), 5);

        // [][][][][][C1][][C][][C0][P][][][][C]
        let mr = time_board.move_player(7);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 3);
        assert_eq!(mr.distance, 7);

        assert_eq!(time_board.current_player(), play_order.players().nth(1));
        assert_eq!(time_board.index_of_current_player(), 5);
        assert_eq!(time_board.index_of_next_player(), 9);

        // [][][][][][C][][C][][C0][P1][][][][C]
        let mr = time_board.move_player(5);
        assert_eq!(mr.pieces, vec![Piece::single_position()]);
        assert_eq!(mr.collects, 2);
        assert_eq!(mr.distance, 5);

        assert_eq!(time_board.current_player(), play_order.players().nth(0));
        assert_eq!(time_board.index_of_current_player(), 9);
        assert_eq!(time_board.index_of_next_player(), 10);

        // [][][][][][C][][C][][C][P1][][][][C2]
        let mr = time_board.move_player(8);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 1);
        assert_eq!(mr.distance, 5);

        assert_eq!(time_board.current_player(), play_order.players().nth(1));
        assert_eq!(time_board.index_of_current_player(), 10);
        assert_eq!(time_board.index_of_next_player(), 14);
        assert!(! time_board.is_game_over());

        // [][][][][][C][][C][][C][P][][][][C12]
        let mr = time_board.move_player(4);
        assert_eq!(mr.pieces, vec![]);
        assert_eq!(mr.collects, 1);
        assert_eq!(mr.distance, 4);

        assert_eq!(time_board.current_player(), None);
        assert!(time_board.is_game_over());
    }
}
