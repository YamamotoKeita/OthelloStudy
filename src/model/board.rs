use crate::model::point::*;
use crate::{Direction, shift_points, PlayerType};

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    pub player1_stones: Points,
    pub player2_stones: Points,
    pub stone_count: u32,
    pub next_player: PlayerType,
    // pub placeable_points: Points,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            player1_stones: 0,
            player2_stones: 0,
            stone_count: 4,
            next_player: PlayerType::First,
        };

        board.set_stone_xy(PlayerType::First, 3, 4);
        board.set_stone_xy(PlayerType::First, 4, 3);
        board.set_stone_xy(PlayerType::Second, 3, 3);
        board.set_stone_xy(PlayerType::Second, 4, 4);
        return board;
    }

    /*
     * Place a stone and reverse opposite stones.
     */
    pub fn place_stone(&self, player: PlayerType, point: Points) -> Board {
        let mut reversed: u64 = 0;
        let mut player_stones = self.get_stones(player);
        let mut opponent_stones = self.get_stones(player.opposite());

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

        let stone_count = self.stone_count + 1;
        let player1_stones = match player {
            PlayerType::First => player_stones,
            PlayerType::Second => opponent_stones,
        };
        let player2_stones = match player {
            PlayerType::First => opponent_stones,
            PlayerType::Second => player_stones,
        };

        Board {
            player1_stones,
            player2_stones,
            stone_count,
            next_player
        }
    }

    #[inline(always)]
    fn set_stone_xy(&mut self, player: PlayerType, x: u32, y: u32) {
        self.set_stone(player, xy_to_point(x, y));
    }

    #[inline(always)]
    fn set_stone(&mut self, player: PlayerType, point: Points) {
        *self.get_stones_ref(player) |= point;
    }

    #[inline(always)]
    pub fn has_stone(&self, player: PlayerType, x: u32, y: u32) -> bool {
        let point = xy_to_point(x, y);
        self.has_stone_at_point(player, point)
    }

    #[inline(always)]
    pub fn has_stone_at_point(&self, player: PlayerType, point: Points) -> bool {
        self.get_stones(player) & point > 0
    }

    pub fn count_stones(&self, player: PlayerType) -> u32 {
        self.get_stones(player).count_ones()
    }

    pub fn can_play(&self, player: PlayerType) -> bool {
        self.placeable_points(player) != 0
    }

    pub fn can_place(&self, player: PlayerType, point: Points) -> bool {
        self.placeable_points(player) & point != 0
    }

    pub fn placeable_points(&self, player: PlayerType) -> Points {
        let player_stones = self.get_stones(player);
        let opponent_stones = self.get_stones(player.opposite());

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
    pub fn get_stones(&self, player: PlayerType) -> u64 {
        match player {
            PlayerType::First => self.player1_stones,
            PlayerType::Second => self.player2_stones,
        }
    }

    #[inline(always)]
    pub fn get_stones_ref(&mut self, player: PlayerType) -> &mut u64 {
        match player {
            PlayerType::First => &mut self.player1_stones,
            PlayerType::Second => &mut self.player2_stones,
        }
    }
}
