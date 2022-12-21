use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;
use crate::model::evaluation::Evaluation;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let count = board.count_stones(board.player);

        if board.player == PlayerType::First {
            count
        } else {
            -count
        }
    }
}