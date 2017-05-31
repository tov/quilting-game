//! Model for a quilting game.

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rand;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod result;
pub mod position;
pub mod piece;
pub mod quilt_board;
pub mod time_board;
pub mod piece_board;

