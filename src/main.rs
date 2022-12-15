use crate::model::board::*;
use crate::cui_view::CuiView;
use crate::model::direction::Direction;
use crate::model::point::{shift_points, points_to_str, to_point, xy_to_point};
use crate::model::stone_color::StoneColor;

mod model;
mod cui_view;
mod game_manager;
mod sandbox;

fn main() {
    let board = Board::new();
    let points = board.placeable_points(StoneColor::First);

    println!("{}", points_to_str(points));
}
