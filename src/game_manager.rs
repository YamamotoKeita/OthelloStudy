use crate::board::{Board, Point, StoneColor};

pub trait Player {
    fn play(&self, board: &Board, color: StoneColor) -> Point;
}

pub trait OthelloView {}

struct GameManager<T: Player, R: Player> {
    board: Board,
    first_player: T,
    second_player: R,
    turn_color: StoneColor,
}

impl<T: Player, R: Player> GameManager<T, R> {
    pub fn new(first_player: T, second_player: R) -> GameManager<T, R> {
        GameManager {
            board: Board::new(),
            first_player,
            second_player,
            turn_color: StoneColor::Black,
        }
    }

    pub fn start_game(&mut self) {
        let mut player: &dyn Player = &self.first_player;

        // while true {
        //     let point = player.play(&self.board, self.turn_color);
        //     self.board.place_stone_at_point(self.turn_color, point);
        //     player = self.get_player(StoneColor::Black);
        // }

        while true {
            // view.wait_next_move(board, turn_color)

            // Place a stone
            let point = player.play(&self.board, self.turn_color);
            self.board.place_stone_at_point(self.turn_color, point);

            // view.place_stone(point, before, after)

            let opposite = self.turn_color.opposite();

            // Change to next player
            if self.board.can_play(opposite) {
                self.turn_color = opposite;
                // player = self.get_player(opposite);
            } else {
                // view.skipped(color)
            }

            // The game is over
            if !self.board.can_play(self.turn_color) {
                break;
            }
        }

        //view.game_end(board)

        println!("The game is over.");
    }

    fn get_player(&self, color: StoneColor) -> &dyn Player {
        match color {
            StoneColor::Black => &self.first_player,
            StoneColor::White => &self.second_player,
        }
    }
}
