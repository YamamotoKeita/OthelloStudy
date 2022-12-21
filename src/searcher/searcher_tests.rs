#[cfg(test)]

mod searcher_tests {
    use crate::evaluator::stone_count::StoneCountEvaluator;
    use crate::{Board, NegaAlpha};
    use crate::searcher::alpha_beta::AlphaBeta;
    use crate::searcher::game_tree_searcher::GameTreeSearcher;

    #[test]
    fn first_move() {
        test_first_move(&alpha_beta());
        test_first_move(&nega_alpha());
    }

    fn test_first_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new();
        let result = searcher.evaluate_children(&board, 1);
        assert_eq!(result.len(), 4);
        for (_, value) in result {
            assert_eq!(value, 3);
        }
    }

    #[test]
    fn second_move() {
        test_second_move(&alpha_beta());
        test_second_move(&nega_alpha());
    }

    fn test_second_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_moves("F5");
        let result = searcher.evaluate_children(&board, 1);
        assert_eq!(result.len(), 3);
        for (_, value) in result {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn end_of_game() {

    }

    #[test]
    fn includes_skipping_turn() {

    }

    #[test]
    fn end_of_game_in_middle_stages() {

    }


    fn alpha_beta() -> AlphaBeta<StoneCountEvaluator> {
        AlphaBeta::new(StoneCountEvaluator::new())
    }

    fn nega_alpha() -> NegaAlpha<StoneCountEvaluator> {
        NegaAlpha::new(StoneCountEvaluator::new())
    }
}
