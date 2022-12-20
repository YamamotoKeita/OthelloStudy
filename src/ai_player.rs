use crate::{Board};
use crate::game_manager::Player;
use crate::model::point::Points;
use crate::searcher::Searcher;

pub struct AiPlayer<T: Searcher> {
    searcher: T,
}

impl <T: Searcher> AiPlayer<T> {
    pub fn new(searcher: T) -> AiPlayer<T> {
        AiPlayer { searcher }
    }
}

impl <T: Searcher> Player for AiPlayer<T> {
    fn play(&self, board: &Board) -> Points {
        self.searcher.search(board, 8)
    }
}