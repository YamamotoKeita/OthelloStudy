use crate::{Board};
use crate::evaluator::Evaluator;
use crate::evaluator::game_end::GameEndEvaluator;


pub struct GameEvaluator<T: Evaluator> {
    end_evaluator: GameEndEvaluator,
    evaluator: T,
}

impl <T: Evaluator> Evaluator for GameEvaluator<T> {
    fn evaluate(&self, board: &Board) -> i32 {
        if board.is_game_end() {
            self.end_evaluator.evaluate(board)
        } else {
            self.evaluator.evaluate(board)
        }
    }
}

impl <T: Evaluator> GameEvaluator<T> {
    pub fn new(evaluator: T) -> GameEvaluator<T> {
        GameEvaluator {
            end_evaluator: GameEndEvaluator::new(),
            evaluator,
        }
    }
}