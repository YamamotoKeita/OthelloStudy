use crate::model::points::*;
use crate::{Direction, shift_points, PlayerType};

/// Representation of Othello board.
#[derive(Clone, Copy)]
pub struct Board {
    pub player1_stones: Points,
    pub player2_stones: Points,
    pub stone_count: u32,
    pub player: PlayerType,
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
            player: PlayerType::First,
            placeable_points: Board::placeable_points(player1_stones, player2_stones),
        }
    }

    #[allow(dead_code)]
    pub fn new_by_moves(text: &str) -> Board {
        let mut board = Board::new();

        let len = text.len() / 2;

        for n in 0..len {
            let from = 2 * n;
            let to = from + 2;
            let point_text = &text[from..to];
            let point = to_point(point_text);
            board = board.place_stone(point);
        }

        board
    }

    #[allow(dead_code)]
    pub fn new_by_text(text: &str, player: PlayerType) -> Board {
        let mut player1_stones: Points = 0;
        let mut player2_stones: Points = 0;

        for (y, line) in text.trim().split("\n").enumerate() {
            for (x, c) in line.split(" ").enumerate() {
                let point = xy_to_point(x as u32, y as u32);
                if c == "●" {
                    player1_stones |= point;
                } else if c == "○" {
                    player2_stones |= point;
                }
            }
        }

        let (player_stones, opponent_stones) = match player {
            PlayerType::First => (player1_stones, player2_stones),
            PlayerType::Second => (player2_stones, player1_stones),
            PlayerType::None => panic!("Use a player when there is no player."),
        };

        Board {
            player1_stones,
            player2_stones,
            stone_count: player1_stones.count_ones() + player2_stones.count_ones(),
            player,
            placeable_points: Board::placeable_points(player_stones, opponent_stones),
        }
    }

    /*
     * Place a stone and reverse opposite stones.
     */
    pub fn place_stone(&self, point: Points) -> Board {
        let mut reversed: u64 = 0;
        let mut player_stones = self.get_stones(self.player);
        let mut opponent_stones = self.get_stones(self.player.opposite());

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

        let (player1_stones, player2_stones) = match self.player {
            PlayerType::First => (player_stones, opponent_stones),
            PlayerType::Second => (opponent_stones, player_stones),
            PlayerType::None => panic!("Use a player when there is no player."),
        };

        let placeable_points = Board::placeable_points(opponent_stones, player_stones);

        Board {
            player1_stones,
            player2_stones,
            stone_count,
            player: self.player.opposite(),
            placeable_points,
        }
    }

    /*
     * Skips the turn of self.player, then if there are no placeable points, set player None.
     */
    pub fn skip_turn(&self) -> Board {
        let mut new_player = self.player.opposite();
        let opponent = self.player;

        let player_stones = self.get_stones(new_player);
        let opponent_stones = self.get_stones(opponent);
        let placeable_points = Board::placeable_points(player_stones, opponent_stones);

        if placeable_points == 0 {
            new_player = PlayerType::None;
        }

        Board {
            player1_stones: self.player1_stones,
            player2_stones: self.player2_stones,
            stone_count: self.stone_count,
            player: new_player,
            placeable_points
        }
    }

    pub fn is_game_end(&self) -> bool {
        self.player == PlayerType::None
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
            let mut tmp = targets & shift_points_without_guard(player_stones, *direction);
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
            PlayerType::None => panic!("Use a player when there is no player."),
        }
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub fn get_stones_ref(&mut self, player: PlayerType) -> &mut u64 {
        match player {
            PlayerType::First => &mut self.player1_stones,
            PlayerType::Second => &mut self.player2_stones,
            PlayerType::None => panic!("Use a player when there is no player."),
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let mut result = "".to_string();

        result.push_str("  A B C D E F G H\n");

        for y in 0..=7 {
            result.push_str(&((y + 1).to_string() + " "));

            for x in 0..=7 {
                let stone = if self.has_stone(PlayerType::First, x, y) {
                    "●"
                } else if self.has_stone(PlayerType::Second, x, y) {
                    "○"
                } else {
                    "□"
                };
                result.push_str(stone);
                result.push_str(" ");
            }
            result.push_str("\n");
        }


        return result;
    }
}
