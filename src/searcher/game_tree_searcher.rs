use crate::{Board, Points};
use crate::searcher::Searcher;

pub trait GameTreeSearcher {
    fn evaluate_children(&self, board: &Board, max_depth: u32) -> Vec<(Points, i32)>;

    fn search_best_move(&self, board: &Board, max_depth: u32) -> Points {
        let mut result: Option<Points> = None;
        let mut max_score = i32::MIN;

        let children = self.evaluate_children(board, max_depth);

        for (points, value) in children {
            if value > max_score {
                result = Some(points);
                max_score = value;
            }
        }
        return result.unwrap();
    }
}

impl Searcher for dyn GameTreeSearcher {
    fn search(&self, board: &Board, max_depth: u32) -> Points {
        return self.search_best_move(board, max_depth);
    }
}