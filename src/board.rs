pub enum StoneColor {
    Black,
    White,
}

/// Represents a point on the Othello board as 1 bit of a 64 bit integer.
/// The 64 bits of integer correspond to the 8 x 8 squares of the board.
type Point = i64;

/// Representation of Othello board.
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
        *self.get_stones(color) |= point;
    }

    pub fn remove_stone(&mut self, color: StoneColor, point: Point) {
        //
    }

    #[inline(always)]
    pub fn to_point(x: i32, y: i32) -> Point {
        1_i64 << y * 8 + x
    }

    #[inline(always)]
    pub fn get_stones(&mut self, color: StoneColor) -> &mut i64 {
        match color {
            StoneColor::Black => &mut self.black_stones,
            StoneColor::White => &mut self.white_stones,
        }
    }

    /*
     *     A   B   C   D   E   F   G   H
     *   +---+---+---+---+---+---+---+---+
     * 1 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 2 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 3 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 4 |   |   |   | ● | ○ |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 5 |   |   |   | ○ | ● |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 6 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 7 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     * 8 |   |   |   |   |   |   |   |   |
     *   +---+---+---+---+---+---+---+---+
     *
     */
    pub fn to_str(&self) -> String {
        let mut result = "".to_string();

        let border = "  +---+---+---+---+---+---+---+---+\n";
        result.push_str("    A   B   C   D   E   F   G   H\n");

        for x in 0..=7 {
            result.push_str(border);
            result.push_str(&((x + 1).to_string() + " "));
            for y in 0..=7 {
                result.push_str("| ");

                let point = Board::to_point(x, y);
                let stone = if (self.black_stones & point) == point {
                    "○"
                } else if (self.white_stones & point) == point {
                    "●"
                } else {
                    " "
                };
                result.push_str(stone);
                result.push_str(" ");
            }
            result.push_str("|\n");
        }
        result.push_str(border);

        return result;
    }
}
