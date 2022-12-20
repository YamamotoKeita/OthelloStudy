use crate::Board;

mod cell_weight;
mod stone_count;
mod placeable_point;
mod open_count;
pub mod game_evaluator;
pub mod simple_prediction;

pub trait Evaluator {
    /*
     * Rules of evaluation is the followings.
     *
     * - 0 for even, positive value for first player advantage, negative value for second player advantage
     */
    fn evaluate(&self, board: &Board) -> i32;
}