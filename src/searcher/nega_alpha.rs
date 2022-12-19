use std::cmp::max;
use crate::{Board, PlayerType, Points};
use crate::evaluator::Evaluator;
use crate::searcher::Searcher;

struct NegaAlpha<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for NegaAlpha<T> {
    fn search(&self, _board: &Board) -> Points {
        todo!()
    }
}

impl <T: Evaluator> NegaAlpha<T> {

    fn nega_alpha(&self, mut board: Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            let sign = if board.next_player.unwrap() == PlayerType::First { 1 } else { -1 };
            return self.evaluator.evaluate(&board) * sign;
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return -self.nega_alpha(board, depth, -beta, -alpha);
        }

        for point in 0_u64..64 {
            if (point & board.placeable_points) == 0 { continue; }

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
