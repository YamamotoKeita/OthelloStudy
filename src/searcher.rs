mod alpha_beta;

use crate::{Board, Points};

pub trait Searcher {
    fn search(&self, board: &Board) -> Points;
}