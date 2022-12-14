use crate::board::Board;
use crate::cui_view::CuiView;

mod board;
mod cui_view;
mod game_manager;
mod sandbox;

fn main() {
    let board = Board::new();
    let view = CuiView::new();

    println!("{}", view.to_str(&board))
}
