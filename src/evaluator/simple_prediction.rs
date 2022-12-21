use crate::{Board};
use crate::evaluator::cell_weight::CellWeightEvaluator;
use crate::evaluator::Evaluator;
use crate::evaluator::open_count::OpenCountEvaluator;
use crate::evaluator::placeable_point::PlaceablePointEvaluator;
use crate::model::evaluation::Evaluation;


pub struct SimplePrediction {
    cell_weight: CellWeightEvaluator,
    placeable_point: PlaceablePointEvaluator,
    open_count: OpenCountEvaluator,
}

impl Evaluator for SimplePrediction {
    fn evaluate(&self, board: &Board) -> Evaluation {
        let cell_weight = self.cell_weight.evaluate(board);
        let placeable_point = self.placeable_point.evaluate(board);
        let open_count = self.open_count.evaluate(board);

        cell_weight + placeable_point * 2 + open_count
    }
}

impl SimplePrediction {
    pub fn new() -> SimplePrediction {
        SimplePrediction {
            cell_weight: CellWeightEvaluator{},
            placeable_point: PlaceablePointEvaluator{},
            open_count: OpenCountEvaluator{},
        }
    }
}