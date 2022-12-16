use crate::model::point::*;
use crate::{Direction, shift_points, StoneColor};

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    pub player1_stones: Points,
    pub player2_stones: Points,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            player1_stones: 0,
            player2_stones: 0,
        };

        board.set_stone_xy(StoneColor::First, 3, 4);
        board.set_stone_xy(StoneColor::First, 4, 3);
        board.set_stone_xy(StoneColor::Second, 3, 3);
        board.set_stone_xy(StoneColor::Second, 4, 4);
        return board;
    }

    pub fn place_stone(&self, color: StoneColor, point: Points) -> Board {
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

        match color {
            StoneColor::First => Board {player1_stones: player_stones, player2_stones: opponent_stones},
            StoneColor::Second => Board {player1_stones: opponent_stones, player2_stones: player_stones}
        }
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

    #[allow(dead_code)]
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

    pub fn can_place(&self, color: StoneColor, point: Points) -> bool {
        self.placeable_points(color) & point != 0
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
            for _ in 0..5 {
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
            StoneColor::First => self.player1_stones,
            StoneColor::Second => self.player2_stones,
        }
    }

    #[inline(always)]
    pub fn get_stones_ref(&mut self, color: StoneColor) -> &mut u64 {
        match color {
            StoneColor::First => &mut self.player1_stones,
            StoneColor::Second => &mut self.player2_stones,
        }
    }
}
