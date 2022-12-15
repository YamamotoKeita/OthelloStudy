use std::io;

use crate::{Board, StoneColor, to_point};
use crate::game_manager::Player;
use crate::model::point::Points;

pub struct CuiPlayer {
}

impl Player for CuiPlayer {
    fn play(&self, board: &Board, color: StoneColor) -> Points {
        println!("Enter the location where you place a stone. (e.g. \"4C\")");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            if let Some(point) = to_point(&input) {
                return point;
            } else {
                println!("Invalid input");
            }
        }
    }
}