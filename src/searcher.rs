pub mod alpha_beta;
pub mod nega_alpha;
pub mod searcher_tests;
mod game_tree_searcher;

use crate::{Board, Points};

pub trait Searcher {
    fn search(&self, board: &Board, max_depth: u32) -> Points;
}