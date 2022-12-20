use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;

const MAGNITUDE: i32 = 100;

pub struct StoneCountEvaluator {}

impl Evaluator for StoneCountEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let count1 = board.count_stones(PlayerType::First);
        let count2 = board.count_stones(PlayerType::Second);
        (count1 as i32 - count2 as i32) * MAGNITUDE
    }
}

impl StoneCountEvaluator {
    pub fn new() -> StoneCountEvaluator {
        StoneCountEvaluator {}
    }
}