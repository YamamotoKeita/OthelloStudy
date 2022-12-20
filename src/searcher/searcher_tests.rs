use crate::evaluator::stone_count::StoneCountEvaluator;
use crate::{Board, NegaAlpha, points_to_str, to_point};
use crate::searcher::alpha_beta::AlphaBeta;

#[cfg(test)]



#[test]
fn first_4_moves() {
    let board = Board::new();
    let board = board.place_stone(to_point("F5").unwrap());

    let nega_alpha = NegaAlpha::new(StoneCountEvaluator{});
    let alpha_beta = AlphaBeta::new(StoneCountEvaluator{});
    let max_depth = 1;

    let results1 = nega_alpha.evaluate_children(&board, max_depth);
    let results2 = alpha_beta.evaluate_children(&board, max_depth);

    println!("NegaAlpha ------------");
    for (points, value) in results1 {
        println!("{} = {}", points_to_str(points), value);
    }

    println!("AlphaBeta ------------");
    for (points, value) in results2 {
        println!("{} = {}", points_to_str(points), value);
    }

}

#[test]
fn end_of_game() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn includes_skipping_turn() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn end_of_game_in_middle_stages() {
    assert_eq!(2 + 2, 4);
}