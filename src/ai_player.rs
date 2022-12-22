use crate::{Board};
use crate::game_manager::Player;
use crate::model::points::Points;
use crate::searcher::Searcher;

pub struct AiPlayer<T: Searcher> {
    searcher: T,
}

impl <T: Searcher> AiPlayer<T> {
    pub fn new(searcher: T) -> AiPlayer<T> {
        AiPlayer { searcher }
    }

    fn get_depth(&self, stone_count: u32) -> u32 {
        let turn = stone_count - 3;
        return if 48 <= turn {
            16
        } else if 46 <= turn {
            12
        } else if 44 <= turn {
            10
        } else if 40 <= turn {
            9
        } else if 8 <= turn {
            7
        } else if 2 <= turn {
            8
        } else {
            9
        }
    }
}

impl <T: Searcher> Player for AiPlayer<T> {
    fn play(&self, board: &Board) -> Points {
        self.searcher.search(board, self.get_depth(board.stone_count() as u32))
    }

    fn message_before_play(&self, board: &Board) -> Option<String> {
        Some(format!("Turn={}, Depth={}", board.stone_count() - 3, self.get_depth(board.stone_count() as u32)))
    }

}