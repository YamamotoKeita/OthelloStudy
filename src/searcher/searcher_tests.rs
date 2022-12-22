#[cfg(test)]

#[allow(unused_imports)]
mod searcher_tests {
    use crate::evaluator::stone_count::StoneCountEvaluator;
    use crate::{Board, CuiView, NegaAlpha, PlayerType, Points, points_to_str};
    use crate::evaluator::stone_count_nyanyan::StoneCountNyanyanEvaluator;
    use crate::model::evaluation::Evaluation;
    use crate::searcher::alpha_beta::AlphaBeta;
    use crate::searcher::game_tree_searcher::GameTreeSearcher;
    use crate::searcher::mini_max::MiniMax;
    use crate::searcher::nega_max_nyanyan::NegaMaxNyanyan;

    fn searchers() -> Vec<Box<dyn GameTreeSearcher>> {
        vec![
            Box::new(mini_max()),
            Box::new(alpha_beta()),
            Box::new(nega_alpha()),
            Box::new(nega_max_nyanyan()),
        ]
    }

    fn mini_max() -> MiniMax<StoneCountEvaluator> {MiniMax::new(StoneCountEvaluator::new())}
    fn alpha_beta() -> AlphaBeta<StoneCountEvaluator> {AlphaBeta::new(StoneCountEvaluator::new())}
    fn nega_alpha() -> NegaAlpha<StoneCountEvaluator> {NegaAlpha::new(StoneCountEvaluator::new())}
    fn nega_max_nyanyan() -> NegaMaxNyanyan<StoneCountNyanyanEvaluator> {NegaMaxNyanyan::new(StoneCountNyanyanEvaluator::new())}

    #[test]
    fn first_move() {
        for searcher in searchers() {
            test_first_move(&*searcher, 1);
            test_first_move(&*searcher, 2);
            test_first_move(&*searcher, 3);
            test_first_move(&*searcher, 7);
        }
    }
    fn test_first_move(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new();
        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 4);
        let evaluation1 = result[0].1;
        assert_eq!(result[1].1, evaluation1);
        assert_eq!(result[2].1, evaluation1);
        assert_eq!(result[3].1, evaluation1);
    }

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
    fn dn_from_black_consistent() {
        for n in 1..=5 {
            test_dn_from_black_consistent(n);
        }
    }
    fn test_dn_from_black_consistent(depth: u32) {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4");
            let result = searcher.evaluate_next_moves(&board, depth);
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
    fn dn_from_white_consistent() {
        for n in 1..=5 {
            test_dn_from_white_consistent(n);
        }
    }
    fn test_dn_from_white_consistent(depth: u32) {
        let mut results: Vec<Vec<(Points, Evaluation)>> = vec![];
        for searcher in searchers() {
            let board = Board::new_by_moves("F5F4G3");
            let result = searcher.evaluate_next_moves(&board, depth);
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
            test_d1_end_from_black(&*searcher, 1);
            test_d1_end_from_black(&*searcher, 2);
            test_d1_end_from_black(&*searcher, 3);
        }
    }
    fn test_d1_end_from_black(searcher: &dyn GameTreeSearcher, depth: u32) {
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

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 64);
    }

    #[test]
    fn d1_end_from_white() {
        for searcher in searchers() {
            test_d1_end_from_white(&*searcher, 1);
            test_d1_end_from_white(&*searcher, 2);
            test_d1_end_from_white(&*searcher, 3);
        }
    }
    fn test_d1_end_from_white(searcher: &dyn GameTreeSearcher, depth: u32) {
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

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 64);
    }

    #[test]
    fn d2_end_from_black() {
        for searcher in searchers() {
            test_d2_end_from_black(&*searcher, 2);
            test_d2_end_from_black(&*searcher, 3);
            test_d2_end_from_black(&*searcher, 4);
        }
    }
    fn test_d2_end_from_black(searcher: &dyn GameTreeSearcher, depth: u32) {
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

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 58);
    }

    #[test]
    fn d2_end_from_white() {
        for searcher in searchers() {
            test_d2_end_from_white(&*searcher, 2);
            test_d2_end_from_white(&*searcher, 3);
            test_d2_end_from_white(&*searcher, 4);
        }
    }
    fn test_d2_end_from_white(searcher: &dyn GameTreeSearcher, depth: u32) {
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

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 58);
    }

    /*
        Root
        Black(●) Turn
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ○ ● ●
        ● ● ● ● ● ● ● ○
        ● ● ● ● ○ □ □ □

        D1
        White(○) Turn
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ○ ● ●   ● ● ● ● ● ○ ● ●
        ● ● ● ● ● ● ● ○   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● □ □   ● ● ● ● ○ □ □ ●

        D2
        Black Turn
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ○ ● ●   ● ● ● ● ● ○ ● ●
        ● ● ● ● ● ● ○ ○   ● ● ● ● ● ○ ● ●
        ● ● ● ● ● ● □ ○   ● ● ● ● ○ ○ □ ●

        D3
        End
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ○ ● ●   ● ● ● ● ● ○ ● ●
        ● ● ● ● ● ● ● ○   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ○   ● ● ● ● ● ● ● ●
        61 - 3 = 58       63 - 1 = 62
     */
    #[test]
    fn d3_2branches_end_from_black() {
        for searcher in searchers() {
            test_d3_2branches_end_from_black(&*searcher, 3);
            test_d3_2branches_end_from_black(&*searcher, 4);
            test_d3_2branches_end_from_black(&*searcher, 5);
        }
    }
    fn test_d3_2branches_end_from_black(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ○ ● ●
● ● ● ● ● ● ● ○
● ● ● ● ○ □ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, 62);
        assert_eq!(result[1].1, 58);
    }

    #[test]
    fn d3_2branches_end_from_white() {
        for searcher in searchers() {
            test_d3_2branches_end_from_white(&*searcher, 3);
            test_d3_2branches_end_from_white(&*searcher, 4);
            test_d3_2branches_end_from_white(&*searcher, 5);
        }
    }
    fn test_d3_2branches_end_from_white(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ● ○ ○
○ ○ ○ ○ ○ ○ ○ ●
○ ○ ○ ○ ● □ □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, 62);
        assert_eq!(result[1].1, 58);
    }

    /*
        Root
        Black(●) Turn
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ○ ○
        ● ● ● ● ● ● ● ●
        ● ● ● ● ○ □ □ □

        D1
        White(○) Turn
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ○ ○
        ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● □ □

        D2
        Black Turn        White Turn (Black skipped)
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ○ ○   ● ● ● ● ● ● ○ ○
        ● ● ● ● ● ● ○ ●   ● ● ● ● ● ● ● ○
        ● ● ● ● ● ● ○ □   ● ● ● ● ● ● □ ○

        D3
        End
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ● ●
        ● ● ● ● ● ● ○ ○   ● ● ● ● ● ● ○ ○
        ● ● ● ● ● ● ○ ●   ● ● ● ● ● ● ○ ○
        ● ● ● ● ● ● ● ●   ● ● ● ● ● ● ○ ○
        61 - 3 = 58       58 - 6 = 52
     */
    #[test]
    fn d3_2branches_end_from_black_2() {
        for searcher in searchers() {
            test_d3_2branches_end_from_black_2(&*searcher, 3);
            test_d3_2branches_end_from_black_2(&*searcher, 4);
            test_d3_2branches_end_from_black_2(&*searcher, 5);
        }
    }
    fn test_d3_2branches_end_from_black_2(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ○ ○
● ● ● ● ● ● ● ●
● ● ● ● ○ □ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 52);
    }

    #[test]
    fn d2_2branches_including_skip_from_black() {
        for searcher in searchers() {
            test_d2_2branches_including_skip_from_black(&*searcher);
        }
    }
    fn test_d2_2branches_including_skip_from_black(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ●
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ □ □ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, -46);
    }

    #[test]
    fn d2_2branches_including_skip_from_white() {
        for searcher in searchers() {
            test_d2_2branches_including_skip_from_white(&*searcher);
        }
    }
    fn test_d2_2branches_including_skip_from_white(searcher: &dyn GameTreeSearcher) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ○
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● □ □ □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, 2);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, -46);
    }

    #[test]
    fn d3_including_skip_from_black() {}

    #[test]
    fn d3_including_skip_from_white() {}

    #[test]
    fn d1_middle_end_from_black() {
        for searcher in searchers() {
            test_d1_middle_end_from_black(&*searcher, 1);
            test_d1_middle_end_from_black(&*searcher, 2);
            test_d1_middle_end_from_black(&*searcher, 3);
        }
    }
    fn test_d1_middle_end_from_black(searcher: &dyn GameTreeSearcher, depth: u32) {
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

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 63);
    }

    #[test]
    fn d1_middle_end_from_white() {
        for searcher in searchers() {
            test_d1_middle_end_from_white(&*searcher, 1);
            test_d1_middle_end_from_white(&*searcher, 2);
            test_d1_middle_end_from_white(&*searcher, 3);
        }
    }
    fn test_d1_middle_end_from_white(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ● □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, 63);
    }

    #[test]
    fn d2_middle_end_from_black() {
        for searcher in searchers() {
            test_d2_middle_end_from_black(&*searcher, 2);
            test_d2_middle_end_from_black(&*searcher, 3);
            test_d2_middle_end_from_black(&*searcher, 4);
        }
    }
    fn test_d2_middle_end_from_black(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ○ ○ ○ ○ ○
○ ○ ○ ● ○ □ □ □
", PlayerType::First);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, -63);
    }

    #[test]
    fn d2_middle_end_from_white() {
        for searcher in searchers() {
            test_d2_middle_end_from_white(&*searcher, 2);
            test_d2_middle_end_from_white(&*searcher, 3);
            test_d2_middle_end_from_white(&*searcher, 4);
        }
    }
    fn test_d2_middle_end_from_white(searcher: &dyn GameTreeSearcher, depth: u32) {
        let board = Board::new_by_text("
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ● ● ● ● ●
● ● ● ○ ● □ □ □
", PlayerType::Second);

        let result = searcher.evaluate_next_moves(&board, depth);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, -63);
    }
}
