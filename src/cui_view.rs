use crate::game_manager::OthelloView;
use crate::model::board::Board;
use crate::model::point::Points;
use crate::StoneColor;

pub struct CuiView {
    player1_stone: String,
    player2_stone: String,
}

impl CuiView {
    pub fn new() -> CuiView {
        CuiView {
            player1_stone: "○".to_string(),
            player2_stone: "●".to_string(),
        }
    }

    pub fn to_str(&self, board: &Board) -> String {
        let mut result = "".to_string();

        let border = "  +---+---+---+---+---+---+---+---+\n";
        result.push_str("    A   B   C   D   E   F   G   H\n");

        for y in 0..=7 {
            result.push_str(border);
            result.push_str(&((y + 1).to_string() + " "));

            for x in 0..=7 {
                result.push_str("| ");

                let stone = if board.has_stone(StoneColor::First, x, y) {
                    &self.player1_stone
                } else if board.has_stone(StoneColor::Second, x, y) {
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

    pub fn get_stone_ref(&self, color: StoneColor) -> &String {
        match color {
            StoneColor::First => &self.player1_stone,
            StoneColor::Second => &self.player2_stone,
        }
    }
}

impl OthelloView for CuiView  {
    fn wait_next_move(&self, board: &Board, color: StoneColor) {
        let text = self.to_str(board);
        println!("{}", text);

        let stone = self.get_stone_ref(color);
        println!("{} turn", stone);
    }

    fn place_stone(&self, point: Points, before: &Board, after: &Board) {

    }

    fn skipped(&self, color: StoneColor) {
        let stone = self.get_stone_ref(color);
        println!("{} turn is skipped.", stone);
    }

    fn game_end(&self, board: &Board) {
        println!("Game End");
        let first_count = board.count_stones(StoneColor::First).to_string();
        let second_count = board.count_stones(StoneColor::Second).to_string();
        println!("{}: {}, {}: {}", self.player1_stone, first_count, self.player2_stone, second_count);
    }
}
