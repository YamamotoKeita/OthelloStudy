use crate::game_manager::OthelloView;
use crate::model::board::Board;
use crate::model::points::Points;
use crate::PlayerType;

pub struct CuiView {
    player1_stone: String,
    player2_stone: String,
}

impl CuiView {
    pub fn new() -> CuiView {
        CuiView {
            player1_stone: "●".to_string(),
            player2_stone: "○".to_string(),
        }
    }

    pub fn to_str(&self, board: &Board) -> String {
        let mut result = "".to_string();

        let border = "  +---+---+---+---+---+---+---+---+";
        result.push_str("    A   B   C   D   E   F   G   H\n");

        for y in 0..=7 {
            result.push_str(border);
            result.push_str("\n");
            result.push_str(&((y + 1).to_string() + " "));

            for x in 0..=7 {
                result.push_str("| ");

                let stone = if board.has_stone(PlayerType::First, x, y) {
                    &self.player1_stone
                } else if board.has_stone(PlayerType::Second, x, y) {
                    &self.player2_stone
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

    pub fn get_stone_ref(&self, player: PlayerType) -> &String {
        match player {
            PlayerType::First => &self.player1_stone,
            PlayerType::Second => &self.player2_stone,
            PlayerType::None => panic!("Use a player when there is no player."),
        }
    }
}

impl OthelloView for CuiView  {
    fn wait_next_move(&self, board: &Board) {
        let text = self.to_str(board);
        println!("{}", text);

        let stone = self.get_stone_ref(board.player);
        println!("{} Turn", stone);
    }

    fn send_message(&self, text: String) {
        println!("{}", text);
    }

    fn place_stone(&self, _point: Points, _before: &Board, _after: &Board) {
    }

    fn skipped(&self, player: PlayerType) {
        let stone = self.get_stone_ref(player);
        println!("{} turn is skipped.", stone);
    }

    fn game_end(&self, board: &Board) {
        println!("Game End");
        let first_count = board.count_stones(PlayerType::First).to_string();
        let second_count = board.count_stones(PlayerType::Second).to_string();
        println!("{}: {}, {}: {}", self.player1_stone, first_count, self.player2_stone, second_count);
    }
}
