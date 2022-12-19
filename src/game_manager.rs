use crate::model::board::Board;
use crate::model::point::Points;
use crate::PlayerType;

pub trait Player {
    fn play(&self, board: &Board) -> Points;
}

pub trait OthelloView {
    fn wait_next_move(&self, board: &Board);
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

        loop {
            self.view.wait_next_move(&board);

            // Place a stone
            let player_type = board.player.unwrap();
            let player = self.get_player(player_type);
            let point = player.play(&board);
            let mut new_board = board.place_stone(point);
            self.view.place_stone(point, &board, &new_board);

            if new_board.placeable_points == 0 {
                new_board.skip_turn();

                if new_board.is_game_end() {
                    break;
                } else {
                    self.view.skipped(player_type.opposite());
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
