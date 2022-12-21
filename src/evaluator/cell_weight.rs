use crate::{Board, Points};
use crate::evaluator::Evaluator;
use crate::model::evaluation::Evaluation;

const CELL_WEIGHT: [i32; 64] = [
    30, -12,   0, -1, -1,  0, -12,  30,
   -12, -15,  -3, -3, -3, -3, -15, -12,
     0,  -3,   0, -1, -1,  0,  -3,   0,
    -1,  -3,  -1, -1, -1, -1,  -3,  -1,
    -1,  -3,  -1, -1, -1, -1,  -3,  -1,
     0,  -3,   0, -1, -1,  0,  -3,   0,
   -12,  -15, -3, -3, -3, -3, -15, -12,
    30,  -12,  0, -1, -1,  0, -12,  30,
];

pub struct CellWeightEvaluator {}

impl Evaluator for CellWeightEvaluator {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let mut first = 0_i32;
        let mut second = 0_i32;

        for n in 0..CELL_WEIGHT.len() {
            let mask: Points = 1 << (63 - n);
            if board.player1_stones & mask != 0 {
                first += CELL_WEIGHT[n];
            }
            if board.player2_stones & mask != 0 {
                second += CELL_WEIGHT[n];
            }
        }

        first - second
    }
}