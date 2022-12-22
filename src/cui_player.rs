use std::io;

use crate::{Board, text_to_point_safely};
use crate::game_manager::Player;
use crate::model::points::Points;

pub struct CuiPlayer {
}

impl CuiPlayer {
    pub fn new() -> CuiPlayer {
        CuiPlayer{}
    }
}

impl Player for CuiPlayer {
    fn play(&self, board: &Board) -> Points {
        println!("Enter the place. (e.g. \"F5\" or \"f5\")");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");

            if let Some(point) = text_to_point_safely(&input.trim()) {
                if board.can_place(point) {
                    return point;
                } else {
                    println!("You can't put a stone there.")
                }
            } else {
                println!("Invalid input.");
            }
        }
    }

    fn message_before_play(&self, _board: &Board) -> Option<String> {
        None
    }
}