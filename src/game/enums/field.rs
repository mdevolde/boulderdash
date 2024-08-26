use crate::game::{interfaces::entity::Entity, wall::Wall};

pub enum Field {
    Empty,
    Dirt,
    Wall(Wall),
    Entity(Box<dyn Entity>),
    Exit,
}
