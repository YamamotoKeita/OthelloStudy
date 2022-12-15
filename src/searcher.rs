use crate::Board;
use crate::evaluator::Evaluator;

pub trait Searcher {
    fn search(&self, board: &Board, evaluator: Evaluator) -> i32;
}