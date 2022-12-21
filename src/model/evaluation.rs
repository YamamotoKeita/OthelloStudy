pub type Evaluation = i32;

// Adds or subtracts 1, because MIN and MAX make overflow when they negate.
pub const EVALUATION_MIN: Evaluation = i32::MIN + 1;
pub const EVALUATION_MAX: Evaluation = i32::MAX - 1;
