use std::slice::Iter;
use self::Direction::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    UpperRight,
    Right,
    LowerRight,
    Down,
    LowerLeft,
    Left,
    UpperLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [Up, UpperRight, Right, LowerRight, Down, LowerLeft, Left, UpperLeft];
        DIRECTIONS.iter()
    }
}
