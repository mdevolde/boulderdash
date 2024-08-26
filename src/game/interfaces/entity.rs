use std::any::Any;

use crate::game::grid::Grid;

use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait UnclonableEntity: Movable + Collidable + Renderable {
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn update(&mut self, grid: &mut Grid);
}

pub trait Entity: UnclonableEntity {
    fn clone_box(&self) -> Box<dyn Entity>;
}

impl<T> Entity for T
where
    T: 'static + UnclonableEntity + Clone,
{
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Box<dyn Entity> {
        self.clone_box()
    }
}
