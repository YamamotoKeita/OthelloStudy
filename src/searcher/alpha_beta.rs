use std::cmp::{max, min};
use crate::{Board, PlayerType, POINT_ITERATOR, Points};
use crate::evaluator::Evaluator;
use crate::searcher::game_tree_searcher::GameTreeSearcher;
use crate::searcher::Searcher;

pub struct AlphaBeta<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for AlphaBeta<T> {
    fn search(&self, board: &Board, max_depth: u32) -> Points {
        return self.search_best_move(board, max_depth);
    }
}

#[allow(dead_code)]
impl <T: Evaluator> AlphaBeta<T> {
    pub fn new(evaluator: T) -> AlphaBeta<T> {
        AlphaBeta {
            evaluator
        }
    }

    fn alpha_beta(&self, mut board: Board, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {

        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            return self.evaluator.evaluate(&board);
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return self.alpha_beta(board, depth, alpha, beta);
        }

        return if board.player.unwrap() == PlayerType::First {
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.alpha_beta(new_board, depth - 1, alpha, beta);
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
                let score = self.alpha_beta(new_board, depth - 1, alpha, beta);
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
    fn evaluate_children(&self, board: &Board, max_depth: u32) -> Vec<(Points, i32)> {
        // Adds or subtracts 1, because MIN and MAX make overflow when they negate.
        let alpha = i32::MIN + 1;
        let beta = i32::MAX - 1;

        let mut result: Vec<(Points, i32)> = vec![];

        for point in *POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                let score = self.alpha_beta(new_board, max_depth - 1, alpha, beta);
                result.push((point, score));
            }
        }

        return result;
    }
}