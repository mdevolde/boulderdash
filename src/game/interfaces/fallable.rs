use crate::game::grid::Grid;

use super::super::enums::movement::Movement;

pub trait Fallable {
    fn fall(&mut self, grid: &Grid);
    fn is_falling(&self, grid: &Grid) -> Option<Movement>;
}