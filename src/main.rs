use crate::ai_player::AiPlayer;
use crate::cui_player::CuiPlayer;
use crate::model::board::*;
use crate::model::points::*;
use crate::cui_view::CuiView;
use crate::evaluator::game_evaluator::GameEvaluator;
use crate::model::direction::Direction;
use crate::model::player_type::PlayerType;
use crate::game_manager::*;
use crate::evaluator::simple_prediction::SimplePrediction;
use crate::searcher::nega_alpha::NegaAlpha;

mod model;
mod cui_view;
mod game_manager;
mod cui_player;
mod searcher;
mod evaluator;
mod util;
mod ai_player;
mod sandbox;

fn main() {
    let player1 = CuiPlayer::new();
    // let player2 = CuiPlayer::new();

    let game_evaluator = GameEvaluator::new(SimplePrediction::new());
    let nega_alpha = NegaAlpha::new(game_evaluator);
    let player2 = AiPlayer::new(nega_alpha);

    let view = CuiView::new();

    let mut manager = GameManager::new( player1, player2, view);
    manager.start_game();
}
