use crate::Direction;

/// Represents a point on the Othello board as 1 bit of a 64 bit integer.
/// The 64 bits of integer correspond to the 8 x 8 squares of the board.
pub type Point = u64;

#[inline(always)]
pub fn xy_to_point(x: u32, y: u32) -> Point {
    let x_shift = 7 - x;
    let y_shift = 7 - y;
    1_u64 << y_shift * 8 + x_shift
}

/*
 * Convert a location text (such as "1A", "3C") to a point.
 */
pub fn to_point(text: &str) -> Option<Point> {
    if text.len() != 2 || !text.is_ascii() {
        return None;
    }

    let mut chars = text.chars();
    let number = chars.next().unwrap();
    let alphabet = chars.next().unwrap();

    let y: u32;
    if let Some(i) = number.to_digit(10) {
        y = i - 1;
    } else {
        return None;
    }

    let x: u32;
    if let Some(i) = alphabet_to_digit(alphabet) {
        x = i;
    } else {
        return None;
    }

    println!("x={}, y={}", x, y);

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

pub fn point_to_str(point: Point) -> String {
    let mut result = "".to_string();

    let border = "  +---+---+---+---+---+---+---+---+\n";
    result.push_str("    A   B   C   D   E   F   G   H\n");

    for y in 0..=7 {
        result.push_str(border);
        result.push_str(&((y + 1).to_string() + " "));

        for x in 0..=7 {
            result.push_str("| ");
            let stone = if (point & xy_to_point(x, y)) != 0 {
                "◉"
            } else {
                " "
            };
            result.push_str(stone);
            result.push_str(" ");
        }
        result.push_str("|\n");
    }
    result.push_str(border);

    return result;
}