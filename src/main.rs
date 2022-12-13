use crate::board::{Board, StoneColor};

mod board;

fn main() {
    let mut board = Board::new();
    // board.place_stone(StoneColor::Black, 0, 0);
    // board.place_stone(StoneColor::Black, 1, 1);
    // board.place_stone(StoneColor::White, 2, 2);

    println!("{}", board.to_str());
}
