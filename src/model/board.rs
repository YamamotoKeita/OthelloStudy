
use crate::model::point::Point;
use crate::StoneColor;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    UpperRight,
    Right,
    LowerRight,
    Down,
    LowerLeft,
    Left,
    UpperLeft,
}

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
    pub fn to_point(x: i32, y: i32) -> Point {
        let x_shift = 7 - x;
        let y_shift = 7 - y;
        1_u64 << y_shift * 8 + x_shift
    }

    /*
     * Move the point to the next square in the specified direction.
     */
    #[inline(always)]
    pub fn move_point(point: Point, direction: Direction) -> Point {
        // Move by bit-shifting and mask bits that is out of the board.
        match direction {
            Direction::Up               => (point << 8) & 0xffffffffffffff00,
            Direction::UpperRight       => (point << 7) & 0x7f7f7f7f7f7f7f00,
            Direction::Right            => (point >> 1) & 0x7f7f7f7f7f7f7f7f,
            Direction::LowerRight       => (point >> 9) & 0x007f7f7f7f7f7f7f,
            Direction::Down             => (point >> 8) & 0x00ffffffffffffff,
            Direction::LowerLeft        => (point >> 7) & 0x00fefefefefefefe,
            Direction::Left             => (point << 1) & 0xfefefefefefefefe,
            Direction::UpperLeft        => (point << 9) & 0xfefefefefefefe00,
        }
    }

    #[inline(always)]
    pub fn place_stone(&mut self, color: StoneColor, point: Point) {
        // TODO Not yet implemented
    }

    #[inline(always)]
    pub fn set_stone_xy(&mut self, color: StoneColor, x: i32, y: i32) {
        self.set_stone(color, Board::to_point(x, y));
    }

    #[inline(always)]
    pub fn set_stone(&mut self, color: StoneColor, point: Point) {
        *self.get_stones_ref(color) |= point;
    }

    #[inline(always)]
    pub fn has_stone(&self, color: StoneColor, x: i32, y: i32) -> bool {
        let point = Board::to_point(x, y);
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
