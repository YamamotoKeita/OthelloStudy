use crate::model::point::{Point, to_point};
use crate::{Direction, StoneColor};

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    black_stones: u64,
    white_stones: u64,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            black_stones: 0,
            white_stones: 0,
        };

        board.set_stone_xy(StoneColor::White, 3, 3);
        board.set_stone_xy(StoneColor::White, 4, 4);
        board.set_stone_xy(StoneColor::Black, 3, 4);
        board.set_stone_xy(StoneColor::Black, 4, 3);
        return board;
    }

    #[inline(always)]
    pub fn place_stone(&mut self, color: StoneColor, point: Point) {
        // TODO Not yet implemented
    }

    #[inline(always)]
    pub fn set_stone_xy(&mut self, color: StoneColor, x: i32, y: i32) {
        self.set_stone(color, to_point(x, y));
    }

    #[inline(always)]
    pub fn set_stone(&mut self, color: StoneColor, point: Point) {
        *self.get_stones_ref(color) |= point;
    }

    #[inline(always)]
    pub fn has_stone(&self, color: StoneColor, x: i32, y: i32) -> bool {
        let point = to_point(x, y);
        self.has_stone_at_point(color, point)
    }

    #[inline(always)]
    pub fn has_stone_at_point(&self, color: StoneColor, point: Point) -> bool {
        self.get_stones(color) & point > 0
    }

    pub fn remove_stone(&mut self, color: StoneColor, point: Point) {
        // TODO Not yet implemented
    }

    pub fn can_play(&self, color: StoneColor) -> bool {
        // TODO Not yet implemented
        true
    }

    #[inline(always)]
    pub fn get_stones(&self, color: StoneColor) -> u64 {
        match color {
            StoneColor::Black => self.black_stones,
            StoneColor::White => self.white_stones,
        }
    }

    #[inline(always)]
    pub fn get_stones_ref(&mut self, color: StoneColor) -> &mut u64 {
        match color {
            StoneColor::Black => &mut self.black_stones,
            StoneColor::White => &mut self.white_stones,
        }
    }
}
