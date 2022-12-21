use crate::{Board, PlayerType, POINT_ITERATOR};
use crate::evaluator::Evaluator;
use crate::model::evaluation::Evaluation;
use crate::searcher::game_tree_searcher::GameTreeSearcher;

pub struct MiniMax<T: Evaluator> {
    evaluator: T,
}

#[allow(dead_code)]
impl <T: Evaluator> MiniMax<T> {
    pub fn new(evaluator: T) -> MiniMax<T> {
        MiniMax {
            evaluator
        }
    }

    fn mini_max(&self, board: &Board, depth: u32, player: PlayerType) -> i32 {

        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            return self.evaluator.evaluate(&board) * player.sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            return self.mini_max(&board.skip_turn(), depth, player);
        }

        return if board.player == player {
            let mut max = i32::MIN;
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.mini_max(&new_board, depth - 1, player);
                if score > max {
                    max = score;
                }
            }
            max
        } else {
            let mut min = i32::MAX;
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.mini_max(&new_board, depth - 1, player);
                if score < min {
                    min = score;
                }
            }
            min
        }
    }
}

impl <T: Evaluator> GameTreeSearcher for MiniMax<T> {
    fn evaluate_child_board(&self, board: &Board, child_board: &Board, depth: u32) -> Evaluation {
        self.mini_max(child_board, depth, board.player)
    }
}