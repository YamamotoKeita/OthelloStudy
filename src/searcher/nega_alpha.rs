use std::cmp::max;
use crate::{Board, POINT_ITERATOR, Points};
use crate::evaluator::Evaluator;
use crate::searcher::game_tree_searcher::GameTreeSearcher;
use crate::searcher::Searcher;

pub struct NegaAlpha<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for NegaAlpha<T> {
    fn search(&self, board: &Board, max_depth: u32) -> Points {
        return self.search_best_move(board, max_depth);
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
            return self.evaluator.evaluate(&board) * board.player.sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return -self.nega_alpha(board, depth, -beta, -alpha);
        }

        for point in *POINT_ITERATOR {
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

impl <T: Evaluator> GameTreeSearcher for NegaAlpha<T> {
    fn evaluate_next_moves(&self, board: &Board, max_depth: u32) -> Vec<(Points, i32)> {
        // Adds or subtracts 1, because MIN and MAX make overflow when they negate.
        let alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;

        let mut result: Vec<(Points, i32)> = vec![];

        for point in *POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                let score = -self.nega_alpha(new_board, max_depth - 1, -beta, -alpha);
                result.push((point, score));
            }
        }

        return result;
    }
}