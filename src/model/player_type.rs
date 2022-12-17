use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PlayerType {
    First,
    Second,
}
impl PlayerType {
    pub fn opposite(&self) -> PlayerType {
        match self {
            PlayerType::First => PlayerType::Second,
            PlayerType::Second => PlayerType::First,
        }
    }
}

impl fmt::Display for PlayerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerType::First => write!(f, "First"),
            PlayerType::Second => write!(f, "Second"),
        }
    }
}