use std::any::Any;

use crate::game::{action::Action, grid::Grid};

use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait Entity: Movable + Collidable + Renderable {
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn update(&self, grid: &Grid) -> Vec<Action>;
}
