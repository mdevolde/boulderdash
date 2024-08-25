use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait Entity: Movable + Collidable + Renderable {}