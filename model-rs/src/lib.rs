#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate rand;

pub mod piece;
pub mod quilt_board;
pub mod time_board;
pub mod piece_board;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
