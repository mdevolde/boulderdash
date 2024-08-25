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

    pub fn edit_position(&self, position: (i32, i32)) -> (i32, i32) {
        match self {
            Movement::MoveUp => (position.0, position.1 - 1),
            Movement::MoveDown => (position.0, position.1 + 1),
            Movement::MoveLeft => (position.0 - 1, position.1),
            Movement::MoveRight => (position.0 + 1, position.1),
            Movement::Afk => position,
        }
    }
}
