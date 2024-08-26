use std::fmt;
use std::any::Any;

use crate::game::{action::Action, grid::Grid};

use super::{collidable::Collidable, movable::Movable, renderable::Renderable};

pub trait Entity: Movable + Collidable + Renderable {
    fn get_type(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn update(&self, grid: &Grid) -> Vec<Action>;

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl fmt::Debug for dyn Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}
