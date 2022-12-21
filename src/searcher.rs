pub mod alpha_beta;
pub mod nega_alpha;
pub mod searcher_tests;
pub mod game_tree_searcher;
pub mod mini_max;

use crate::{Board, Points};

pub trait Searcher {
    fn search(&self, board: &Board, max_depth: u32) -> Points;
}