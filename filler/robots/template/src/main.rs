use log::{LevelFilter};
use glam::UVec2;
use lib::{Board, Piece, PlaceInput, Robot, setup_logger};

fn main() {
    setup_logger(LevelFilter::Debug);

    let mut bot = MyBot::new();
    bot.run();
}

struct MyBot {
}

impl MyBot {
    fn new() -> Self {
        Self {}
    }
}

impl Robot for MyBot {
    fn place(&mut self, input: &PlaceInput) -> Option<UVec2> {
        let board = input.board;
        let piece = input.piece;

        Some(UVec2::new(9, 2))
    }
}