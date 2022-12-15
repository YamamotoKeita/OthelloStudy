use crate::Direction;

pub const MASK_BOTTOM_ZERO: u64         = 0xffffffffffffff00;
pub const MASK_LEFT_BOTTOM_ZERO: u64    = 0x7f7f7f7f7f7f7f00;
pub const MASK_LEFT_ZERO: u64           = 0x7f7f7f7f7f7f7f7f;
pub const MASK_TOP_LEFT_ZERO: u64       = 0x007f7f7f7f7f7f7f;
pub const MASK_TOP_ZERO: u64            = 0x00ffffffffffffff;
pub const MASK_TOP_RIGHT_ZERO: u64      = 0x00fefefefefefefe;
pub const MASK_RIGHT_ZERO: u64          = 0xfefefefefefefefe;
pub const MASK_RIGHT_BOTTOM_ZERO: u64   = 0xfefefefefefefe00;
pub const MASK_LEFT_RIGHT_ZERO: u64     = 0x7e7e7e7e7e7e7e7e;
pub const MASK_TOP_BOTTOM_ZERO: u64     = 0x00FFFFFFFFFFFF00;
pub const MASK_ALL_SIDES_ZERO: u64      = 0x007e7e7e7e7e7e00;

/// Represents specific points on the Othello board as bits in a 64 bit integer.
/// The 64 bits of integer correspond to the 64 (8 x 8) squares of Othello board.
pub type Points = u64;

#[inline(always)]
pub fn xy_to_point(x: u32, y: u32) -> Points {
    let x_shift = 7 - x;
    let y_shift = 7 - y;
    1_u64 << y_shift * 8 + x_shift
}

/*
 * Convert a location text (such as "1A", "3C") to a point.
 */
pub fn to_point(text: &str) -> Option<Points> {
    if text.len() != 2 || !text.is_ascii() {
        return None;
    }

    let mut chars = text.chars();
    let alphabet = chars.next().unwrap();
    let number = chars.next().unwrap();

    let x: u32;
    if let Some(i) = alphabet_to_digit(alphabet) {
        x = i;
    } else {
        return None;
    }

    let y: u32;
    if let Some(i) = number.to_digit(10) {
        y = i - 1;
    } else {
        return None;
    }

    return Some(xy_to_point(x, y));
}

#[inline(always)]
fn alphabet_to_digit(alphabet: char) -> Option<u32> {
    let i = alphabet as u32;

    if 'A' as u32 <= i && i <= 'H' as u32 {
        return Some(i - 'A' as u32);
    }
    if 'a' as u32 <= i && i <= 'h' as u32 {
        return Some(i - 'a' as u32);
    }

    return None;
}

/*
 * Move the point to the next square in the specified direction.
 */
#[inline(always)]
pub fn shift_points(points: Points, direction: Direction) -> Points {
    // Move by bit-shifting and mask bits that is out of the board.
    match direction {
        Direction::Up               => (points << 8) & MASK_BOTTOM_ZERO,
        Direction::UpperRight       => (points << 7) & MASK_LEFT_BOTTOM_ZERO,
        Direction::Right            => (points >> 1) & MASK_LEFT_ZERO,
        Direction::LowerRight       => (points >> 9) & MASK_TOP_LEFT_ZERO,
        Direction::Down             => (points >> 8) & MASK_TOP_ZERO,
        Direction::LowerLeft        => (points >> 7) & MASK_TOP_RIGHT_ZERO,
        Direction::Left             => (points << 1) & MASK_RIGHT_ZERO,
        Direction::UpperLeft        => (points << 9) & MASK_RIGHT_BOTTOM_ZERO,
    }
}

#[inline(always)]
pub fn shift_points_without_guard(points: Points, direction: Direction) -> Points {
    match direction {
        Direction::Up               => points << 8,
        Direction::UpperRight       => points << 7,
        Direction::Right            => points >> 1,
        Direction::LowerRight       => points >> 9,
        Direction::Down             => points >> 8,
        Direction::LowerLeft        => points >> 7,
        Direction::Left             => points << 1,
        Direction::UpperLeft        => points << 9,
    }
}

#[allow(dead_code)]
pub fn points_to_str(points: Points) -> String {
    let mut result = "".to_string();

    result.push_str("  A B C D E F G H\n");

    for y in 0..=7 {
        result.push_str(&((y + 1).to_string()));

        for x in 0..=7 {
            result.push_str(" ");
            let square = if (points & xy_to_point(x, y)) != 0 {
                "■"
            } else {
                "□"
            };
            result.push_str(square);
        }
        result.push_str("\n");
    }

    return result;
}