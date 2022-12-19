use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let player = board.player.unwrap();
        let count = board.count_stones(player);

        if player == PlayerType::First {
            count
        } else {
            -count
        }
    }
}