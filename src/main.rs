use crate::cui_player::CuiPlayer;
use crate::model::board::*;
use crate::model::point::*;
use crate::cui_view::CuiView;
use crate::model::direction::Direction;
use crate::model::stone_color::StoneColor;
use crate::game_manager::*;

mod model;
mod cui_view;
mod game_manager;
mod cui_player;
mod searcher;
mod evaluator;
mod util;

fn main() {
    let player1 = CuiPlayer::new();
    let player2 = CuiPlayer::new();
    let view = CuiView::new();

    let mut manager = GameManager::new( player1, player2, view);
    manager.start_game();
}
