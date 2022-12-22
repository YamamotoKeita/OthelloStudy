use std::cmp::max;
use crate::{Board, PlayerType, POINT_ITERATOR};
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

    fn nega_alpha(&self, board: &Board, depth: u32, mut alpha: i32, beta: i32, last_player: PlayerType) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 {
            return self.evaluator.evaluate(&board) * last_player.opposite().sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            let new_board = board.skip_turn();
            return if new_board.is_game_end() {
                self.evaluator.evaluate(&new_board) * last_player.opposite().sign()
            } else {
                -self.nega_alpha(&new_board, depth, -beta, -alpha, last_player)
            }
        }

        for point in *POINT_ITERATOR {
            if !board.can_place(point) { continue; }

            let new_board = board.place_stone(point);
            let score = -self.nega_alpha(&new_board, depth - 1, -beta, -alpha, board.player);

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
    fn evaluate_child_board(&self, board: &Board, child_board: &Board, depth: u32) -> Evaluation {
        -self.nega_alpha(child_board, depth, -EVALUATION_MAX, -EVALUATION_MIN, board.player)
    }
}