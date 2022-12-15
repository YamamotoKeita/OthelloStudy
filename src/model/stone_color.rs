use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum StoneColor {
    First,
    Second,
}
impl StoneColor {
    pub fn opposite(&self) -> StoneColor {
        match self {
            StoneColor::First => StoneColor::Second,
            StoneColor::Second => StoneColor::First,
        }
    }
}

impl fmt::Display for StoneColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoneColor::First => write!(f, "First"),
            StoneColor::Second => write!(f, "Second"),
        }
    }
}