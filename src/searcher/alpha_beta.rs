use std::cmp::max;
use crate::{Board, Points};
use crate::evaluator::Evaluator;
use crate::searcher::Searcher;

struct AlphaBeta<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for AlphaBeta<T> {
    fn search(&self, _board: &Board) -> Points {
        todo!()
    }
}

impl <T: Evaluator> AlphaBeta<T> {

    fn nega_alpha(&self, board: Board, depth: u32, passed: bool, mut alpha: i32, mut beta: i32) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 {
            return self.evaluator.evaluate(&board);
        }

        const UNDEFINED: i32 = i32::MIN;

        let mut score = UNDEFINED;
        let mut max_score = UNDEFINED;

        let placeable_points = board.placeable_points;

        let mut point: Points = 0;
        loop { // Maybe loop is faster than for.
            if point & placeable_points != 0 {
                let new_board = board.place_stone(point);

                score = -self.nega_alpha(new_board, depth - 1, false, -beta, -alpha);

                // pruning
                if score >= beta {
                    return score;
                }

                alpha = max(alpha, score);
                max_score = max(max_score, score);
            }

            point += 1;
            if point > 63 {
                break;
            }
        }


        // パスの処理 手番を交代して同じ深さで再帰する
        if max_score == UNDEFINED {
            // 2回連続パスなら評価関数を実行
            if passed {
                return self.evaluator.evaluate(&board);
            }
            return -self.nega_alpha(board, depth, true, -beta, -alpha);
        }

        return max_score;
    }
}
