#[cfg(test)]

#[allow(unused_imports)]
mod searcher_tests {
    use crate::evaluator::stone_count::StoneCountEvaluator;
    use crate::{Board, CuiView, NegaAlpha, PlayerType, points_to_str};
    use crate::searcher::alpha_beta::AlphaBeta;
    use crate::searcher::game_tree_searcher::GameTreeSearcher;
    use crate::searcher::mini_max::MiniMax;

    fn searchers() -> Vec<Box<dyn GameTreeSearcher>> {
        vec![
            Box::new(MiniMax::new(StoneCountEvaluator::new())),
            Box::new(AlphaBeta::new(StoneCountEvaluator::new())),
            Box::new(NegaAlpha::new(StoneCountEvaluator::new())),
        ]
    }

    #[test]
    fn d1_black_move() {
        for searcher in searchers() {
            test_d1_black_move(&*searcher);
        }
    }

    fn test_d1_black_move(searcher: &dyn GameTreeSearcher) {
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
    fn d1_white_move() {
        for searcher in searchers() {
            test_d1_white_move(&*searcher);
        }
    }

    fn test_d1_white_move(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_moves("F5F4F3");
        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].1, -2);
        assert_eq!(result[1].1, -2);
        assert_eq!(result[2].1, 0);
    }

    #[test]
    fn d1_game_end_from_white() {
        for searcher in searchers() {
            test_d1_game_end_from_white(&*searcher);
        }
    }

    fn test_d1_game_end_from_white(searcher: &dyn GameTreeSearcher) {
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

        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 64);
    }

    #[test]
    fn depth2_from_black() {}

    #[test]
    fn depth3_from_black() {}

    #[test]
    fn depth2_from_white() {}

    #[test]
    fn depth3_from_white() {}

    #[test]
    fn includes_skipping_turn() {

    }

    #[test]
    fn end_of_game_in_middle_stages() {

    }

}
