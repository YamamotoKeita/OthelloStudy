use std::fmt;

#[derive(Clone, Copy)]
pub enum StoneColor {
    Black,
    White,
}

impl StoneColor {
    pub fn opposite(&self) -> StoneColor {
        match self {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        }
    }
}

impl fmt::Display for StoneColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoneColor::Black => write!(f, "Black"),
            StoneColor::White => write!(f, "White"),
        }
    }
}

/// Represents a point on the Othello board as 1 bit of a 64 bit integer.
/// The 64 bits of integer correspond to the 8 x 8 squares of the board.
pub type Point = i64;

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    black_stones: i64,
    white_stones: i64,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            black_stones: 0,
            white_stones: 0,
        };

        board.place_stone(StoneColor::White, 3, 3);
        board.place_stone(StoneColor::White, 4, 4);
        board.place_stone(StoneColor::Black, 3, 4);
        board.place_stone(StoneColor::Black, 4, 3);
        return board;
    }

    #[inline(always)]
    pub fn place_stone(&mut self, color: StoneColor, x: i32, y: i32) {
        self.place_stone_at_point(color, Board::to_point(x, y));
    }

    #[inline(always)]
    pub fn place_stone_at_point(&mut self, color: StoneColor, point: Point) {
        *self.get_stones_ref(color) |= point;
    }

    #[inline(always)]
    pub fn has_stone(&self, color: StoneColor, x: i32, y: i32) -> bool {
        let point = Board::to_point(x, y);
        self.has_stone_at_point(color, point)
    }

    #[inline(always)]
    pub fn has_stone_at_point(&self, color: StoneColor, point: Point) -> bool {
        self.get_stones(color) & point == point
    }

    pub fn remove_stone(&mut self, color: StoneColor, point: Point) {
        // TODO Not yet implemented
    }

    pub fn can_play(&self, color: StoneColor) -> bool {
        // TODO Not yet implemented
        true
    }

    #[inline(always)]
    pub fn to_point(x: i32, y: i32) -> Point {
        1_i64 << y * 8 + x
    }

    #[inline(always)]
    pub fn get_stones(&self, color: StoneColor) -> i64 {
        match color {
            StoneColor::Black => self.black_stones,
            StoneColor::White => self.white_stones,
        }
    }

    #[inline(always)]
    pub fn get_stones_ref(&mut self, color: StoneColor) -> &mut i64 {
        match color {
            StoneColor::Black => &mut self.black_stones,
            StoneColor::White => &mut self.white_stones,
        }
    }
}
