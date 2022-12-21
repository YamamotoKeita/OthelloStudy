use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;
use crate::model::evaluation::Evaluation;

pub struct StoneCountNyanyanEvaluator {
    stone_weight: i32,
}

impl Evaluator for StoneCountNyanyanEvaluator {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let count1 = board.count_stones(PlayerType::First);
        let count2 = board.count_stones(PlayerType::Second);
        let value = (count1 as i32 - count2 as i32) * self.stone_weight;
        value * board.player.sign()
    }
}

#[allow(dead_code)]
impl StoneCountNyanyanEvaluator {
    pub fn new() -> StoneCountNyanyanEvaluator {
        StoneCountNyanyanEvaluator {stone_weight: 1}
    }
}