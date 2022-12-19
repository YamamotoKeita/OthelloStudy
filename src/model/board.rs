use crate::model::point::*;
use crate::{Direction, shift_points, PlayerType};

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    pub player1_stones: Points,
    pub player2_stones: Points,
    pub stone_count: u32,
    pub next_player: Option<PlayerType>,
    pub placeable_points: Points,
}

impl Board {
    pub fn new() -> Board {
        let player1_stones = to_points(&[(3, 4), (4, 3)]);
        let player2_stones = to_points(&[(3, 3), (4, 4)]);

        Board {
            player1_stones,
            player2_stones,
            stone_count: 4,
            next_player: Some(PlayerType::First),
            placeable_points: Board::placeable_points(player1_stones, player2_stones),
        }
    }

    /*
     * Place a stone and reverse opposite stones.
     */
    pub fn place_stone(&self, point: Points) -> Board {
        let player = self.next_player.unwrap();

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

        let (player1_stones, player2_stones) = match player {
            PlayerType::First => (player_stones, opponent_stones),
            PlayerType::Second => (opponent_stones, player_stones),
        };

        let next_player = Some(player.opposite());
        let placeable_points = Board::placeable_points(opponent_stones, player_stones);

        Board {
            player1_stones,
            player2_stones,
            stone_count,
            next_player,
            placeable_points,
        }
    }

    /*
     * Skips the turn of self.next_player, then if there are no placeable points, set next_player None.
     */
    pub fn skip_turn(&mut self) {
        let player = self.next_player.unwrap();
        let next_player = player.opposite();
        self.next_player = Some(next_player);

        let player_stones = self.get_stones(next_player);
        let opponent_stones = self.get_stones(player);
        self.placeable_points = Board::placeable_points(player_stones, opponent_stones);

        if self.placeable_points == 0 {
            self.next_player = None;
        }
    }

    pub fn is_game_end(&self) -> bool {
        self.next_player.is_none()
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

    pub fn count_stones(&self, player: PlayerType) -> i32 {
        self.get_stones(player).count_ones() as i32
    }

    pub fn can_place(&self, point: Points) -> bool {
        self.placeable_points & point != 0
    }

    fn placeable_points(player_stones: Points, opponent_stones: Points) -> Points {
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

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_stones_ref(&mut self, player: PlayerType) -> &mut u64 {
        match player {
            PlayerType::First => &mut self.player1_stones,
            PlayerType::Second => &mut self.player2_stones,
        }
    }
}
