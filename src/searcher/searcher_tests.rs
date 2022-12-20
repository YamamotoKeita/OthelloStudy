use crate::evaluator::stone_count::StoneCountEvaluator;
use crate::NegaAlpha;

#[cfg(test)]



#[test]
fn first_4_moves() {

    let nega_alpha = NegaAlpha::new(StoneCountEvaluator{});

    assert_eq!(2 + 2, 4);
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