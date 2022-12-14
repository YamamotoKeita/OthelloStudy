use std::fmt;

#[derive(Clone, Copy)]
pub enum StoneColor {
    Black,
    White,
}
impl StoneColor {
    pub fn opposite(&self) -> StoneColor {
        match self {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        }
    }
}

impl fmt::Display for StoneColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoneColor::Black => write!(f, "Black"),
            StoneColor::White => write!(f, "White"),
        }
    }
}