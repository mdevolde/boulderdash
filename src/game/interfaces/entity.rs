use std::any::Any;

use crate::game::grid::Grid;

use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait Entity: Movable + Collidable + Renderable {
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn update(&mut self, grid: &mut Grid);
}
