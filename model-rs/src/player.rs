//! Represents players of the game.

use std::slice;
use rand;

/// A game player.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player(usize);

impl Player {
    /// A `usize` identifying the given player.
    ///
    /// Players are numbered starting at 0, so they are suitable as array indices.
    pub fn to_usize(&self) -> usize {
        self.0
    }
}

/// A stack of players ready to play.
///
/// This is placed on the time board to track whose turn it is.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PlayOrder(Vec<Player>);

impl PlayOrder {
    /// Creates a new `PlayOrder` with the given number of players in random order.
    pub fn new(nplayers: usize) -> Self {
        let mut result = Self::new_in_order(nplayers);
        shuffle(&mut rand::thread_rng(), &mut result.0);
        result
    }

    /// Creates a new, empty `PlayOrder`.
    pub fn empty() -> Self {
        PlayOrder(Vec::new())
    }

    /// Creates a new `PlayOrder` with the given number of players in increasing order.
    pub fn new_in_order(nplayers: usize) -> Self {
        let mut stack = Vec::new();
        for i in 0 .. nplayers {
            stack.push(Player(nplayers - i - 1));
        }
        PlayOrder(stack)
    }

    /// Is the given `PlayOrder` empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the number of players in the given `PlayOrder`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets an iterator over the players in play order.
    pub fn players(&self) -> Players {
        Players(self.0.iter())
    }

    /// Pushes a player to go next in the play order.
    pub fn push(&mut self, player: Player) {
        self.0.push(player)
    }

    /// Gets the next player in the play order.
    pub fn pop(&mut self) -> Option<Player> {
        self.0.pop()
    }
}

/// [Fisher-Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle).
fn shuffle<R: rand::Rng, T>(rng: &mut R, v: &mut Vec<T>) {
    use rand::distributions::{IndependentSample, Range};

    for i in (1 .. v.len()).rev() {
        let range = Range::new(0, i);
        let j = range.ind_sample(rng);
        v.swap(i, j);
    }
}

impl<'a> IntoIterator for &'a PlayOrder {
    type IntoIter = Players<'a>;
    type Item = &'a Player;

    fn into_iter(self) -> Self::IntoIter {
        self.players()
    }
}

/// An iterator over the `Player`s in a `PlayOrder`.
pub struct Players<'a>(slice::Iter<'a, Player>);

impl<'a> Iterator for Players<'a> {
    type Item = &'a Player;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a> ExactSizeIterator for Players<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> DoubleEndedIterator for Players<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}