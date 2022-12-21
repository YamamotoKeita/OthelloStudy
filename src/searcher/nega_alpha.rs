use std::cmp::max;
use crate::{Board, POINT_ITERATOR};
use crate::evaluator::Evaluator;
use crate::model::evaluation::{Evaluation, EVALUATION_MAX, EVALUATION_MIN};
use crate::searcher::game_tree_searcher::GameTreeSearcher;

pub struct NegaAlpha<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> NegaAlpha<T> {
    pub fn new(evaluator: T) -> NegaAlpha<T> {
        NegaAlpha {
            evaluator
        }
    }

    fn nega_alpha(&self, board: &Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            return self.evaluator.evaluate(&board) * board.player.sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            return -self.nega_alpha(&board.skip_turn(), depth, -beta, -alpha);
        }

        for point in *POINT_ITERATOR {
            if !board.can_place(point) { continue; }

            let new_board = board.place_stone(point);
            let score = -self.nega_alpha(&new_board, depth - 1, -beta, -alpha);

            // pruning
            if score >= beta {
                return score;
            }

            alpha = max(alpha, score);
        }

        return alpha;
    }
}

impl <T: Evaluator> GameTreeSearcher for NegaAlpha<T> {
    fn evaluate_child_board(&self, _board: &Board, child_board: &Board, depth: u32) -> Evaluation {
        -self.nega_alpha(child_board, depth, -EVALUATION_MAX, -EVALUATION_MIN)
    }
}