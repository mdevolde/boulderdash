use crate::game::grid::Grid;

use super::super::enums::movement::Movement;

pub trait Fallable {
    fn fall(&mut self);
    fn is_falling(&self, grid: &Grid) -> Option<Movement>;
}