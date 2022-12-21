use crate::Board;
use crate::model::evaluation::Evaluation;

pub mod cell_weight;
pub mod stone_count;
pub mod placeable_point;
pub mod open_count;
pub mod game_evaluator;
pub mod simple_prediction;
pub mod stone_count_nyanyan;

pub trait Evaluator {
    /*
     * Evaluates a board.
     *
     * The evaluation is for the first(black) player.
     * If the first(black) and second(white) player are equal in the advantage, the evaluation is zero.
     * If the first(black) player has the advantage, the evaluation is a positive number.
     * If the second(white) player has the advantage, the evaluation is a negative number.
     */
    fn evaluate(&self, board: &Board) -> Evaluation;
}