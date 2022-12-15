use std::io;

use crate::{Board, StoneColor, to_point};
use crate::game_manager::Player;
use crate::model::point::Points;

pub struct CuiPlayer {
}

impl CuiPlayer {
    pub fn new() -> CuiPlayer {
        CuiPlayer{}
    }
}

impl Player for CuiPlayer {
    fn play(&self, board: &Board, color: StoneColor) -> Points {
        println!("Enter the place. (e.g. \"4C\" or \"4c\")");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");

            if let Some(point) = to_point(&input.trim()) {
                if board.can_place(color, point) {
                    return point;
                } else {
                    println!("You can't put a stone there.")
                }
            } else {
                println!("Invalid input.");
            }
        }
    }
}