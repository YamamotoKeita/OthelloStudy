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

    pub fn sign(&self) -> i32 {
        match self {
            PlayerType::First => 1,
            PlayerType::Second => -1,
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