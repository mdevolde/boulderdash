pub enum Movement {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Afk
}

impl Movement {
    pub fn from_key(key: &str) -> Movement {
        match key {
            "w" => Movement::MoveUp,
            "s" => Movement::MoveDown,
            "a" => Movement::MoveLeft,
            "d" => Movement::MoveRight,
            _ => Movement::Afk
        }
    }
}
