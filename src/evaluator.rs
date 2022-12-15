use crate::Board;

mod cell_weight;

pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> i32;
}