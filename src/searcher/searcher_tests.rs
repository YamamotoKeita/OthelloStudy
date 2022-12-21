#[cfg(test)]

#[allow(unused_imports)]
mod searcher_tests {
    use crate::evaluator::stone_count::StoneCountEvaluator;
    use crate::{Board, CuiView, NegaAlpha, PlayerType, Points, points_to_str};
    use crate::model::evaluation::Evaluation;
    use crate::searcher::alpha_beta::AlphaBeta;
    use crate::searcher::game_tree_searcher::GameTreeSearcher;
    use crate::searcher::mini_max::MiniMax;

    fn searchers() -> Vec<Box<dyn GameTreeSearcher>> {
        vec![
            Box::new(min_max()),
            Box::new(alpha_beta()),
            Box::new(nega_alpha()),
        ]
    }

    fn min_max() -> MiniMax<StoneCountEvaluator> {MiniMax::new(StoneCountEvaluator::new())}
    fn alpha_beta() -> AlphaBeta<StoneCountEvaluator> {AlphaBeta::new(StoneCountEvaluator::new())}
    fn nega_alpha() -> NegaAlpha<StoneCountEvaluator> {NegaAlpha::new(StoneCountEvaluator::new())}

    #[test]
    fn d1_from_black() {
        for searcher in searchers() {
            test_d1_from_black(&*searcher);
        }
    }
    fn test_d1_from_black(searcher: &dyn GameTreeSearcher) {
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
    fn d1_from_white() {
        for searcher in searchers() {
            test_d1_from_white(&*searcher);
        }
    }
    fn test_d1_from_white(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_moves("F5F4F3");
        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].1, -2);
        assert_eq!(result[1].1, -2);
        assert_eq!(result[2].1, 0);
    }


    #[test]
    fn d2_from_black() {
        for searcher in searchers() {
            test_d2_from_black(&*searcher);
        }
    }
    fn test_d2_from_black(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ○ ●
● ● ● ● ● ● ● ●
● ● ● ● ○ □ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 57);
    }

    #[test]
    fn d2_from_white() {
        for searcher in searchers() {
            test_d2_from_white(&*searcher);
        }
    }
    fn test_d2_from_white(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ● ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ● □ □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 57);
    }

    #[test]
    fn d3_from_black_consistent() {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4");
            let result = searcher.evaluate_next_moves(&board, 3);
            results.push(result);
        }

        let first_result = results.first().unwrap();

        for result in &results {
            assert_eq!(result.len(), first_result.len());
            for n in 0..first_result.len() {
                assert_eq!(result[n].0, first_result[n].0);
                assert_eq!(result[n].1, first_result[n].1);
            }
        }
    }

    #[test]
    fn d4_from_black_consistent() {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4");
            let result = searcher.evaluate_next_moves(&board, 4);
            results.push(result);
        }

        let first_result = results.first().unwrap();

        for result in &results {
            assert_eq!(result.len(), first_result.len());
            for n in 0..first_result.len() {
                assert_eq!(result[n].0, first_result[n].0);
                assert_eq!(result[n].1, first_result[n].1);
            }
        }
    }

    #[test]
    fn d3_from_white_consistent() {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4G3");
            let result = searcher.evaluate_next_moves(&board, 3);
            results.push(result);
        }

        let first_result = results.first().unwrap();

        for result in &results {
            assert_eq!(result.len(), first_result.len());
            for n in 0..first_result.len() {
                assert_eq!(result[n].0, first_result[n].0);
                assert_eq!(result[n].1, first_result[n].1);
            }
        }
    }

    #[test]
    fn d4_from_white_consistent() {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4G3");
            let result = searcher.evaluate_next_moves(&board, 4);
            results.push(result);
        }

        let first_result = results.first().unwrap();
        for result in &results {
            assert_eq!(result.len(), first_result.len());
            for n in 0..first_result.len() {
                assert_eq!(result[n].0, first_result[n].0);
                assert_eq!(result[n].1, first_result[n].1);
            }
        }
    }

    #[test]
    fn d1_end_from_black() {
        for searcher in searchers() {
            test_d1_end_from_black(&*searcher);
        }
    }
    fn test_d1_end_from_black(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ○ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 64);
    }

    #[test]
    fn d1_end_from_white() {
        for searcher in searchers() {
            test_d1_end_from_white(&*searcher);
        }
    }
    fn test_d1_end_from_white(searcher: &dyn GameTreeSearcher) {
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
    fn d2_end_from_black() {
        for searcher in searchers() {
            test_d2_end_from_black(&*searcher);
        }
    }
    fn test_d2_end_from_black(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ○
● ● ● ● ● ● ● ●
● ● ● ● ● ○ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 58);
    }

    #[test]
    fn d2_end_from_white() {
        for searcher in searchers() {
            test_d2_end_from_white(&*searcher);
        }
    }
    fn test_d2_end_from_white(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ●
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ● □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 58);
    }

    #[test]
    fn d3_end_from_black() {}

    #[test]
    fn d3_end_from_white() {}

    #[test]
    fn d2_including_skip_from_black() {}

    #[test]
    fn d2_including_skip_from_white() {}

    #[test]
    fn d3_including_skip_from_black() {}

    #[test]
    fn d3_including_skip_from_white() {}

    #[test]
    fn d2_middle_end_from_black() {
        for searcher in searchers() {
            test_d2_middle_end_from_black(&*searcher);
        }
    }
    fn test_d2_middle_end_from_black(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ○ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 63);
    }

    #[test]
    fn d2_middle_end_from_white() {
    }

    #[test]
    fn d3_middle_end_from_black() {
    }

    #[test]
    fn d3_middle_end_from_white() {
    }

}
