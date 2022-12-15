use crate::game_manager::OthelloView;
use crate::model::board::Board;
use crate::model::point::Points;
use crate::StoneColor;

pub struct CuiView {
    black_stone: String,
    white_stone: String,
}

impl CuiView {
    pub fn new() -> CuiView {
        CuiView {
            black_stone: "○".to_string(),
            white_stone: "●".to_string(),
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

                let stone = if board.has_stone(StoneColor::Black, x, y) {
                    &self.black_stone
                } else if board.has_stone(StoneColor::White, x, y) {
                    &self.white_stone
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
            StoneColor::Black => &self.black_stone,
            StoneColor::White => &self.white_stone,
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
        let black_count = board.count_stones(StoneColor::Black).to_string();
        let white_count = board.count_stones(StoneColor::White).to_string();
        println!("{}: {}, {}: {}", self.black_stone, black_count, self.white_stone, white_count);
    }
}
