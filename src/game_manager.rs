use crate::model::board::Board;
use crate::model::point::Points;
use crate::PlayerType;

pub trait Player {
    fn play(&self, board: &Board, player: PlayerType) -> Points;
}

pub trait OthelloView {
    fn wait_next_move(&self, board: &Board, player: PlayerType);
    fn place_stone(&self, point: Points, before: &Board, after: &Board);
    fn skipped(&self, player: PlayerType);
    fn game_end(&self, board: &Board);
}

pub struct GameManager<P1: Player, P2: Player, View: OthelloView> {
    first_player: P1,
    second_player: P2,
    view: View,
}

impl <P1: Player, P2: Player, View: OthelloView> GameManager<P1, P2, View> {
    pub fn new(first_player: P1, second_player: P2, view: View) -> GameManager<P1, P2, View> {
        GameManager {
            first_player,
            second_player,
            view,
        }
    }

    pub fn start_game(&mut self) {
        let mut board = Board::new();
        let mut turn = PlayerType::First;

        loop {
            self.view.wait_next_move(&board, turn);

            // Place a stone
            let player = self.get_player(turn);
            let point = player.play(&board, turn);
            let new_board = board.place_stone(turn, point);

            self.view.place_stone(point, &board, &new_board);

            let opposite_player = turn.opposite();

            // Change to next player
            if new_board.can_play(opposite_player) {
                turn = opposite_player;
            } else {
                if new_board.can_play(turn) {
                    self.view.skipped(opposite_player)
                } else {
                    // The game is over
                    break;
                }
            }
            board = new_board;
        }

        self.view.game_end(&board);
    }

    fn get_player(&self, player: PlayerType) -> &dyn Player {
        match player {
            PlayerType::First => &self.first_player,
            PlayerType::Second => &self.second_player,
        }
    }
}
