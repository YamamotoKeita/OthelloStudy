use std::cmp::{max, min};
use crate::{Board, PlayerType, POINT_ITERATOR, Points};
use crate::evaluator::Evaluator;
use crate::model::evaluation::{Evaluation, EVALUATION_MAX, EVALUATION_MIN};
use crate::searcher::game_tree_searcher::GameTreeSearcher;

pub struct AlphaBeta<T: Evaluator> {
    evaluator: T,
}

#[allow(dead_code)]
impl <T: Evaluator> AlphaBeta<T> {
    pub fn new(evaluator: T) -> AlphaBeta<T> {
        AlphaBeta {
            evaluator
        }
    }

    fn alpha_beta(&self, mut board: Board, depth: u32, mut alpha: i32, mut beta: i32, player: PlayerType) -> i32 {

        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            return self.evaluator.evaluate(&board) * player.sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return self.alpha_beta(board, depth, alpha, beta, player);
        }

        return if board.player == PlayerType::First {
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.alpha_beta(new_board, depth - 1, alpha, beta, player);
                alpha = max(alpha, score);
                if alpha >= beta {
                    break;
                }
            }
            alpha
        } else {
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.alpha_beta(new_board, depth - 1, alpha, beta, player);
                beta = min(beta, score);
                if alpha >= beta {
                    break;
                }
            }
            beta
        }
    }
}

impl <T: Evaluator> GameTreeSearcher for AlphaBeta<T> {
    fn evaluate_next_moves(&self, board: &Board, max_depth: u32) -> Vec<(Points, Evaluation)> {
        let alpha = EVALUATION_MIN;
        let beta = EVALUATION_MAX;

        let mut result: Vec<(Points, Evaluation)> = vec![];

        for point in *POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                let score = self.alpha_beta(new_board, max_depth - 1, alpha, beta, board.player);
                result.push((point, score));
            }
        }

        return result;
    }
}