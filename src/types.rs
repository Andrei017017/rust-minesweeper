#[derive(Clone)]
pub enum Cell {
    Hidden(bool, Option<char>),
    Revealed(Option<u8>),
    Mine(bool),
    WrongFlag
}

#[derive(Clone, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(PartialEq, Clone)]
pub enum GameState {
    Active,
    Won,
    Lost
}