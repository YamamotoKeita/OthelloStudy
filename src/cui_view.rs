use crate::model::board::Board;
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
}
