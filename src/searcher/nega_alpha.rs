use std::cmp::max;
use crate::{Board, PlayerType, POINT_ITERATOR, Points};
use crate::evaluator::Evaluator;
use crate::searcher::Searcher;

pub struct NegaAlpha<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for NegaAlpha<T> {
    fn search(&self, board: &Board, max_depth: u32) -> Points {

        let mut alpha = i32::MIN;
        let beta = i32::MAX;
        let mut result: Option<Points> = None;
        let mut score: i32;

        for point in POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                score = -self.nega_alpha(new_board, max_depth, -beta, -alpha);

                if alpha < score {
                    alpha = score;
                    result = Some(point);
                }
            }
        }

        return result.unwrap();
    }
}

impl <T: Evaluator> NegaAlpha<T> {
    pub fn new(evaluator: T) -> NegaAlpha<T> {
        NegaAlpha {
            evaluator
        }
    }

    fn nega_alpha(&self, mut board: Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            let sign = if board.player.unwrap() == PlayerType::First { 1 } else { -1 };
            return self.evaluator.evaluate(&board) * sign;
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return -self.nega_alpha(board, depth, -beta, -alpha);
        }

        for point in POINT_ITERATOR {
            if !board.can_place(point) { continue; }

            let new_board = board.place_stone(point);
            let score = -self.nega_alpha(new_board, depth - 1, -beta, -alpha);

            // pruning
            if score >= beta {
                return score;
            }

            alpha = max(alpha, score);
        }

        return alpha;
    }

}
