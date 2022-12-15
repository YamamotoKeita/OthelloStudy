use crate::model::point::*;
use crate::{Direction, shift_points, points_to_str, StoneColor};

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

        board.set_stone_xy(StoneColor::Black, 3, 4);
        board.set_stone_xy(StoneColor::Black, 4, 3);
        board.set_stone_xy(StoneColor::White, 3, 3);
        board.set_stone_xy(StoneColor::White, 4, 4);
        return board;
    }

    pub fn place_stone(&mut self, color: StoneColor, point: Points) {
        let mut reversed: u64 = 0;
        let mut player_stones = self.get_stones(color);
        let mut opponent_stones = self.get_stones(color.opposite());

        for direction in Direction::iterator() {
            let mut line: u64 = 0;
            let mut next_point = shift_points(point, *direction);

            while (next_point != 0) && ((next_point & opponent_stones) != 0) {
                line |= next_point;
                next_point = shift_points(next_point, *direction);
            }

            if (next_point & player_stones) != 0 {
                reversed |= line;
            }
        }

        // Both addition and removal can be done with the XOR operation.
        player_stones ^= point | reversed;
        opponent_stones ^= reversed;

        *self.get_stones_ref(color) = player_stones;
        *self.get_stones_ref(color.opposite()) = opponent_stones;
    }

    #[inline(always)]
    pub fn set_stone_xy(&mut self, color: StoneColor, x: u32, y: u32) {
        self.set_stone(color, xy_to_point(x, y));
    }

    #[inline(always)]
    pub fn set_stone(&mut self, color: StoneColor, point: Points) {
        *self.get_stones_ref(color) |= point;
    }

    #[inline(always)]
    pub fn has_stone(&self, color: StoneColor, x: u32, y: u32) -> bool {
        let point = xy_to_point(x, y);
        self.has_stone_at_point(color, point)
    }

    #[inline(always)]
    pub fn has_stone_at_point(&self, color: StoneColor, point: Points) -> bool {
        self.get_stones(color) & point > 0
    }

    #[inline(always)]
    pub fn remove_stone(&mut self, color: StoneColor, point: Points) {
        *self.get_stones_ref(color) &= !point;
    }

    pub fn count_stones(&self, color: StoneColor) -> u32 {
        self.get_stones(color).count_ones()
    }

    pub fn can_play(&self, color: StoneColor) -> bool {
        self.placeable_points(color) != 0
    }

    pub fn placeable_points(&self, color: StoneColor) -> Points {
        let player_stones = self.get_stones(color);
        let opponent_stones = self.get_stones(color.opposite());

        let horizontal_targets = opponent_stones & MASK_LEFT_RIGHT_ZERO;
        let vertical_targets = opponent_stones & MASK_TOP_BOTTOM_ZERO;
        let diagonal_targets = opponent_stones & MASK_ALL_SIDES_ZERO;

        let vacant_points = !(player_stones | opponent_stones);

        let mut result: u64 = 0;

        for direction in Direction::iterator() {
            let targets = match *direction {
                Direction::Left | Direction::Right => horizontal_targets,
                Direction::Up | Direction::Down => vertical_targets,
                _ => diagonal_targets,
            };

            // Shift the player stones 6 squares and mark where the opponent stones are in the direction of movement.
            let mut tmp= targets & shift_points_without_guard(player_stones, *direction);
            for n in 1..=5 {
                tmp |= targets & shift_points_without_guard(tmp, *direction);
            }

            let placeable_points = vacant_points & shift_points_without_guard(tmp, *direction);

            result |= placeable_points;
        }

        return result;
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
