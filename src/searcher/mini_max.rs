use crate::{Board, PlayerType, POINT_ITERATOR, Points};
use crate::evaluator::Evaluator;
use crate::searcher::game_tree_searcher::GameTreeSearcher;
use crate::searcher::Searcher;

pub struct MiniMax<T: Evaluator> {
    evaluator: T,
}

impl <T: Evaluator> Searcher for MiniMax<T> {
    fn search(&self, board: &Board, max_depth: u32) -> Points {
        return self.search_best_move(board, max_depth);
    }
}

#[allow(dead_code)]
impl <T: Evaluator> MiniMax<T> {
    pub fn new(evaluator: T) -> MiniMax<T> {
        MiniMax {
            evaluator
        }
    }

    fn mini_max(&self, mut board: Board, depth: u32, player: PlayerType) -> i32 {

        // Evaluates a board on a terminal node
        if depth == 0 || board.is_game_end() {
            return self.evaluator.evaluate(&board) * player.sign();
        }

        // Skip and turn change
        if board.placeable_points == 0 {
            board.skip_turn();
            return self.mini_max(board, depth, player);
        }

        return if board.player == player {
            let mut max = i32::MIN;
            for point in *POINT_ITERATOR {
                if !board.can_place(point) { continue; }

                let new_board = board.place_stone(point);
                let score = self.mini_max(new_board, depth - 1, player);
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
                let score = self.mini_max(new_board, depth - 1, player);
                if score < min {
                    min = score;
                }
            }
            min
        }
    }
}

impl <T: Evaluator> GameTreeSearcher for MiniMax<T> {
    fn evaluate_next_moves(&self, board: &Board, max_depth: u32) -> Vec<(Points, i32)> {
        let mut result: Vec<(Points, i32)> = vec![];

        for point in *POINT_ITERATOR {
            if board.can_place(point) {
                let new_board = board.place_stone(point);
                let score = self.mini_max(new_board, max_depth - 1, board.player);
                result.push((point, score));
            }
        }

        return result;
    }
}