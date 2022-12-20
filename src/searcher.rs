pub mod alpha_beta;
pub mod nega_alpha;
pub mod searcher_tests;

use crate::{Board, Points};

pub trait Searcher {
    fn search(&self, board: &Board, max_depth: u32) -> Points;
}