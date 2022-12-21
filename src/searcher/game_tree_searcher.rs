use crate::{Board, POINT_ITERATOR, Points};
use crate::model::evaluation::{Evaluation, EVALUATION_MIN};
use crate::searcher::Searcher;

pub trait GameTreeSearcher {
    fn evaluate_child_board(&self, board: &Board, child_board: &Board, depth: u32) -> Evaluation;

    fn evaluate_next_moves(&self, board: &Board, max_depth: u32) -> Vec<(Points, Evaluation)> {
        let mut result: Vec<(Points, Evaluation)> = vec![];

        for point in *POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                let score = self.evaluate_child_board(board, &new_board, max_depth - 1);
                result.push((point, score));
            }
        }

        return result;
    }
}

impl <T: GameTreeSearcher> Searcher for T {
    fn search(&self, board: &Board, max_depth: u32) -> Points {
        let mut result: Option<Points> = None;
        let mut max_score = EVALUATION_MIN;

        let children = self.evaluate_next_moves(board, max_depth);

        for (points, value) in children {
            if value > max_score {
                result = Some(points);
                max_score = value;
            }
        }
        return result.unwrap();
    }
}