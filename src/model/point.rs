use crate::Direction;

/// Represents a point on the Othello board as 1 bit of a 64 bit integer.
/// The 64 bits of integer correspond to the 8 x 8 squares of the board.
pub type Point = u64;

#[inline(always)]
pub fn to_point(x: i32, y: i32) -> Point {
    let x_shift = 7 - x;
    let y_shift = 7 - y;
    1_u64 << y_shift * 8 + x_shift
}

/*
 * Move the point to the next square in the specified direction.
 */
#[inline(always)]
pub fn move_point(point: Point, direction: Direction) -> Point {
    // Move by bit-shifting and mask bits that is out of the board.
    match direction {
        Direction::Up               => (point << 8) & 0xffffffffffffff00,
        Direction::UpperRight       => (point << 7) & 0x7f7f7f7f7f7f7f00,
        Direction::Right            => (point >> 1) & 0x7f7f7f7f7f7f7f7f,
        Direction::LowerRight       => (point >> 9) & 0x007f7f7f7f7f7f7f,
        Direction::Down             => (point >> 8) & 0x00ffffffffffffff,
        Direction::LowerLeft        => (point >> 7) & 0x00fefefefefefefe,
        Direction::Left             => (point << 1) & 0xfefefefefefefefe,
        Direction::UpperLeft        => (point << 9) & 0xfefefefefefefe00,
    }
}