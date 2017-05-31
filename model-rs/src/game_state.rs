//! The state of a whole quilting game.

use std::default::Default;

use position::Dimension;
use player::{self, PlayerState, PlayOrder};
use piece_board::{PieceBoard, PieceBoardBuilder};
use quilt_board;
use time_board::{TimeBoard, TimeBoardBuilder};

/// The default size of the square needed to get the bonus.
pub const DEFAULT_BONUS_SQUARE_SIZE: usize = 7;

/// Builder for configuring and constructing new games.
#[derive(Debug, Clone)]
pub struct GameBuilder {
    piece_board:       PieceBoardBuilder,
    time_board:        TimeBoardBuilder,
    nplayers:          usize,
    starting_currency: usize,
    quilt_dimension:   Dimension,
    bonus_square_size: Option<usize>,
}

impl GameBuilder {
    /// Creates a new builder with the default parameters.
    pub fn new() -> Self {
        let mut result = Self::empty();
        result.piece_board = PieceBoardBuilder::default();
        result
    }

    /// Creates a new builder whose
    /// [`PieceBoardBuilder`](../piece_board/struct.PieceBoardBuilder.html)
    /// is empty of pieces.
    pub fn empty() -> Self {
        GameBuilder {
            piece_board:       PieceBoardBuilder::empty(),
            time_board:        TimeBoardBuilder::default(),
            nplayers:          player::DEFAULT_NPLAYERS,
            starting_currency: player::DEFAULT_STARTING_CURRENCY,
            quilt_dimension:   Dimension::square(quilt_board::DEFAULT_DIMENSION),
            bonus_square_size: Some(DEFAULT_BONUS_SQUARE_SIZE),
        }
    }

    /// Changes the builder to use the given
    /// [`PieceBoardBuilder`](../piece_board/struct.PieceBoardBuilder.html).
    pub fn piece_board(mut self, piece_board: PieceBoardBuilder) -> Self {
        self.piece_board = piece_board;
        self
    }

    /// Configure the piece board by modifying the
    /// [`PieceBoardBuilder`](../piece_board/struct.PieceBoardBuilder.html).
    pub fn with_piece_board<F>(mut self, k: F) -> Self
        where F: FnOnce(PieceBoardBuilder) -> PieceBoardBuilder
    {
        self.piece_board = k(self.piece_board);
        self
    }

    /// Changes the builder to use the given [`TimeBoard`](../time_board/struct.TimeBoard.html).
    pub fn time_board(mut self, time_board: TimeBoardBuilder) -> Self {
        self.time_board = time_board;
        self
    }

    /// Changes the builder to use the given number of players.
    ///
    /// # Errors
    ///
    /// Panics if `nplayers < 2`.
    pub fn nplayers(mut self, nplayers: usize) -> Self {
        assert!(nplayers >= 2, "Must have at least two players.");
        self.nplayers = nplayers;
        self
    }

    /// Gives each player the given starting amount of money.
    pub fn starting_currency(mut self, starting_currency: usize) -> Self {
        self.starting_currency = starting_currency;
        self
    }

    /// Creates empty quilt boards with the given dimension.
    pub fn quilt_dimension(mut self, quilt_dimension: Dimension) -> Self {
        self.quilt_dimension = quilt_dimension;
        self
    }

    /// Changes the empty quilt to be a square of the given size.
    pub fn quilt_size(mut self, square_size: usize) -> Self {
        self.quilt_dimension = Dimension::square(square_size);
        self
    }

    /// Changes the bonus to be given when reaching a square of the given size.
    pub fn bonus_square_size(mut self, square_size: usize) -> Self {
        self.bonus_square_size = Some(square_size);
        self
    }

    /// Configures the game to have no bonus.
    pub fn no_bonus(mut self) -> Self {
        self.bonus_square_size = None;
        self
    }

    fn build_shuffle(self, shuffle: bool) -> GameState {
        let mut players = Vec::new();

        for _ in 0 .. self.nplayers {
            players.push(PlayerState::new(self.quilt_dimension, self.starting_currency))
        }

        let piece_board;
        let play_order;

        if shuffle {
            piece_board = self.piece_board.build();
            play_order  = PlayOrder::new(self.nplayers);
        } else {
            piece_board = self.piece_board.build_in_order();
            play_order  = PlayOrder::new_in_order(self.nplayers);
        }

        GameState {
            piece_board:       piece_board,
            time_board:        self.time_board.build(play_order),
            players:           players.into_boxed_slice(),
            bonus_square_size: self.bonus_square_size,
        }
    }

    /// Builds the game, shuffling the [`PieceBoard`](../piece_board/struct.PieceBoard.html)
    /// and the play order.
    pub fn build(self) -> GameState {
        self.build_shuffle(true)
    }

    /// Builds the game without shuffling the [`PieceBoard`](../piece_board/struct.PieceBoard.html)
    /// and the play order.
    pub fn build_in_order(self) -> GameState {
        self.build_shuffle(false)
    }
}

impl Default for GameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// The state of the game.
///
/// Configure and construct with [`GameBuilder`](struct.GameBuilder.html).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    /// The board from which pieces are selected.
    piece_board:       PieceBoard,
    /// The board that keeps track of time.
    time_board:        TimeBoard,
    /// Each player’s quilt.
    players:           Box<[PlayerState]>,
    /// The size quilt square to build to get the bonus, if it remains.
    bonus_square_size: Option<usize>,
}

impl GameState {
    /// Is the game over?
    pub fn is_game_over(&self) -> bool {
        self.time_board.is_game_over() || self.piece_board.is_empty()
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameBuilder::new().build()
    }
}