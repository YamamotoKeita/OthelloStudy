use crate::model::board::Board;
use crate::model::point::Points;
use crate::StoneColor;

pub trait Player {
    fn play(&self, board: &Board, color: StoneColor) -> Points;
}

pub trait OthelloView {
    fn wait_next_move(&self, board: &Board, color: StoneColor);
    fn place_stone(&self, point: Points, before: &Board, after: &Board);
    fn skipped(&self, color: StoneColor);
    fn game_end(&self, board: &Board);
}

struct GameManager<P1: Player, P2: Player, View: OthelloView> {
    board: Board,
    first_player: P1,
    second_player: P2,
    view: View,
}

impl<P1: Player, P2: Player, View: OthelloView> GameManager<P1, P2, View> {
    pub fn new(first_player: P1, second_player: P2, view: View) -> GameManager<P1, P2, View> {
        GameManager {
            board: Board::new(),
            first_player,
            second_player,
            view,
        }
    }

    pub fn start_game(&mut self) {
        let mut turn = StoneColor::First;

        while true {
            self.view.wait_next_move(&self.board, turn);

            // Place a stone
            let player = self.get_player(turn);
            let point = player.play(&self.board, turn);
            let before = self.board;
            self.board.set_stone(turn, point);

            self.view.place_stone(point, &before, &self.board);

            let opposite_color = turn.opposite();

            // Change to next player
            if self.board.can_play(opposite_color) {
                turn = opposite_color;
            } else {
                self.view.skipped(opposite_color)
            }

            // The game is over
            if !self.board.can_play(turn) {
                break;
            }
        }

        self.view.game_end(&self.board);
    }

    fn get_player(&self, color: StoneColor) -> &dyn Player {
        match color {
            StoneColor::First => &self.first_player,
            StoneColor::Second => &self.second_player,
        }
    }
}
