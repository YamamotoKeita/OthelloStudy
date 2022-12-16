use crate::{Board, StoneColor};
use crate::evaluator::Evaluator;


pub struct GameEndEvaluator {}

impl Evaluator for GameEndEvaluator {
    fn evaluate(&self, board: &Board) -> i32 {
        let count1 = board.count_stones(StoneColor::First);
        let count2 = board.count_stones(StoneColor::First);
        let mut point = count1 - count2;
        if point > 0 {
            point += 100;
        } else if point < 0 {
            point -= 100;
        }
        point as i32
    }
}