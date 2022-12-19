use crate::{Board, PlayerType, Points};
use crate::evaluator::Evaluator;

pub struct PlaceablePointEvaluator {}

impl Evaluator for PlaceablePointEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        //board.next_player.unwrap()
        let count1 = board.count_stones(PlayerType::First);
        let count2 = board.count_stones(PlayerType::Second);
    }
}