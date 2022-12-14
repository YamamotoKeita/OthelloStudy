use crate::model::board::*;
use crate::cui_view::CuiView;
use crate::model::direction::Direction;
use crate::model::point::{move_point, to_point};
use crate::model::stone_color::StoneColor;

mod model;
mod cui_view;
mod game_manager;
mod sandbox;

fn main() {
    let mut board = Board::new();
    let view = CuiView::new();

    let point = to_point(1, 2);
    let point = move_point(point, Direction::Up);

    board.set_stone(StoneColor::Black, point);

    println!("{}", view.to_str(&board))
}
