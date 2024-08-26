use std::rc::Rc;

use crate::game::{interfaces::entity::Entity, wall::Wall};

#[derive(Clone)]
pub enum Field {
    Empty,
    Dirt,
    Wall(Wall),
    Entity(Rc<dyn Entity>),
    Exit,
}
