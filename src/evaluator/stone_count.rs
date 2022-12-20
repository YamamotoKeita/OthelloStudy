use crate::{Board, PlayerType};
use crate::evaluator::Evaluator;


pub struct StoneCountEvaluator {}

impl Evaluator for StoneCountEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let count1 = board.count_stones(PlayerType::First);
        let count2 = board.count_stones(PlayerType::Second);
        let mut point = count1 - count2;
        if point > 0 {
            point += 100;
        } else if point < 0 {
            point -= 100;
        }
        point as i32
    }
}

impl StoneCountEvaluator {
    pub fn new() -> StoneCountEvaluator {
        StoneCountEvaluator {}
    }
}