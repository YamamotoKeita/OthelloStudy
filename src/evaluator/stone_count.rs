use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;

pub struct StoneCountEvaluator {
    stone_weight: i32,
}

impl Evaluator for StoneCountEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let count1 = board.count_stones(PlayerType::First);
        let count2 = board.count_stones(PlayerType::Second);
        (count1 as i32 - count2 as i32) * self.stone_weight
    }
}

#[allow(dead_code)]
impl StoneCountEvaluator {
    pub fn new() -> StoneCountEvaluator {
        StoneCountEvaluator {stone_weight: 1}
    }
}