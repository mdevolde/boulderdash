use std::any::Any;

use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait Entity: Movable + Collidable + Renderable {
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}
