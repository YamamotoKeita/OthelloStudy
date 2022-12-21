use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let count = board.count_stones(board.player);

        if board.player == PlayerType::First {
            count
        } else {
            -count
        }
    }
}