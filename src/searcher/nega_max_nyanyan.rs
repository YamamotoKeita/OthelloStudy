use std::cmp::max;
use crate::{Board, POINT_ITERATOR};
use crate::evaluator::Evaluator;
use crate::model::evaluation::{Evaluation, EVALUATION_MIN};
use crate::searcher::game_tree_searcher::GameTreeSearcher;

pub struct NegaMaxNyanyan<T: Evaluator> {
    evaluator: T,
}

#[allow(dead_code)]
impl <T: Evaluator> NegaMaxNyanyan<T> {
    pub fn new(evaluator: T) -> NegaMaxNyanyan<T> {
        NegaMaxNyanyan {
            evaluator
        }
    }

    fn nega_max(&self, board: &mut Board, depth: u32, passed: bool) -> i32 {
        // 葉ノードでは評価関数を実行する
        if depth == 0 {
            return self.evaluator.evaluate(board);
        }

        // 葉ノードでなければ子ノード全部に対して再帰する
        let mut max_score = EVALUATION_MIN;
        for point in *POINT_ITERATOR {
            if !board.can_place(point) { continue; }
            let mut new_board = board.place_stone(point);
            max_score = max(max_score, -self.nega_max(&mut new_board, depth - 1, false));
        }

        // パスの処理 手番を交代して同じ深さで再帰する
        if max_score == EVALUATION_MIN {
            // 2回連続パスなら評価関数を実行
            if passed {
                return self.evaluator.evaluate(board);
            }
            board.change_turn();
            return -self.nega_max(board, depth, true);
        }
        return max_score;
    }
}

impl <T: Evaluator> GameTreeSearcher for NegaMaxNyanyan<T> {
    fn evaluate_child_board(&self, _board: &Board, child_board: &Board, depth: u32) -> Evaluation {
        let mut child_board = child_board.clone();
        -self.nega_max(&mut child_board, depth, false)
    }
}