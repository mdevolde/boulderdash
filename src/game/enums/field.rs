use crate::game::{interfaces::entity::Entity, wall::Wall};

pub enum Field {
    Empty,
    Wall(Wall),
    Entity(Box<dyn Entity>),
    Exit,
}
