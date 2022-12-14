use crate::model::board::Board;
use crate::model::point::Point;
use crate::StoneColor;

pub trait Player {
    fn play(&self, board: &Board, color: StoneColor) -> Point;
}

pub trait OthelloView {
    fn wait_next_move(&self, board: &Board, color: StoneColor);
    fn place_stone(&self, point: Point, before: &Board, after: &Board);
    fn skipped(&self, color: StoneColor);
    fn game_end(&self, board: &Board);
}

struct GameManager<P1: Player, P2: Player, View: OthelloView> {
    board: Board,
    first_player: P1,
    second_player: P2,
    view: View,
    turn_color: StoneColor,
}

impl<P1: Player, P2: Player, View: OthelloView> GameManager<P1, P2, View> {
    pub fn new(first_player: P1, second_player: P2, view: View) -> GameManager<P1, P2, View> {
        GameManager {
            board: Board::new(),
            first_player,
            second_player,
            view,
            turn_color: StoneColor::Black,
        }
    }

    pub fn start_game(&mut self) {
        while true {
            self.view.wait_next_move(&self.board, self.turn_color);

            // Place a stone
            let player = self.get_player();
            let point = player.play(&self.board, self.turn_color);
            let before = self.board;
            self.board.set_stone(self.turn_color, point);

            self.view.place_stone(point, &before, &self.board);

            let opposite_color = self.turn_color.opposite();

            // Change to next player
            if self.board.can_play(opposite_color) {
                self.turn_color = opposite_color;
            } else {
                self.view.skipped(opposite_color)
            }

            // The game is over
            if !self.board.can_play(self.turn_color) {
                break;
            }
        }

        self.view.game_end(&self.board);
    }

    fn get_player(&self) -> &dyn Player {
        match self.turn_color {
            StoneColor::Black => &self.first_player,
            StoneColor::White => &self.second_player,
        }
    }
}