#[cfg(test)]

#[allow(unused_imports)]
mod searcher_tests {
    use crate::evaluator::stone_count::StoneCountEvaluator;
    use crate::{Board, CuiView, NegaAlpha, PlayerType, points_to_str};
    use crate::searcher::alpha_beta::AlphaBeta;
    use crate::searcher::game_tree_searcher::GameTreeSearcher;
    use crate::searcher::mini_max::MiniMax;

    #[test]
    fn first_move() {
        test_first_move(&mini_max());
        test_first_move(&alpha_beta());
        test_first_move(&nega_alpha());
    }

    fn test_first_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new();
        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 4);
        for (_, value) in result {
            assert_eq!(value, 3);
        }
    }

    #[test]
    fn black_move() {
        test_black_move(&mini_max());
        test_black_move(&alpha_beta());
        test_black_move(&nega_alpha());
    }

    fn test_black_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_moves("F5F4");
        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].1, 3);
        assert_eq!(result[1].1, 5);
        assert_eq!(result[2].1, 3);
        assert_eq!(result[3].1, 5);
        assert_eq!(result[4].1, 3);
    }

    #[test]
    fn white_move() {
        test_white_move(&mini_max());
        test_white_move(&alpha_beta());
        test_white_move(&nega_alpha());
    }

    fn test_white_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_moves("F5F4F3");
        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].1, -2);
        assert_eq!(result[1].1, -2);
        assert_eq!(result[2].1, 0);
    }

    #[test]
    fn game_end() {
        test_game_end(&mini_max());
    }

    fn test_game_end(_searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ● □
", PlayerType::Second);

        println!("{}", CuiView::new().to_str(&board))

    }

    #[test]
    fn includes_skipping_turn() {

    }

    #[test]
    fn end_of_game_in_middle_stages() {

    }

    fn mini_max() -> MiniMax<StoneCountEvaluator> {
        MiniMax::new(StoneCountEvaluator::new())
    }
    fn alpha_beta() -> AlphaBeta<StoneCountEvaluator> {
        AlphaBeta::new(StoneCountEvaluator::new())
    }
    fn nega_alpha() -> NegaAlpha<StoneCountEvaluator> {
        NegaAlpha::new(StoneCountEvaluator::new())
    }
}
