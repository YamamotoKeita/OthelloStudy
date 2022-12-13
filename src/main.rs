use crate::board::{Board, StoneColor};
use crate::cui_view::CuiView;

mod board;
mod cui_view;

fn main() {
    let mut board = Board::new();
    let view = CuiView::new();

    println!("{}", view.to_str(&board));
}
