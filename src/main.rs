use crate::model::board::*;
use crate::cui_view::CuiView;
use crate::model::direction::Direction;
use crate::model::point::{move_point, point_to_str, to_point, xy_to_point};
use crate::model::stone_color::StoneColor;

mod model;
mod cui_view;
mod game_manager;
mod sandbox;

fn main() {
    to_point("8B");
}
