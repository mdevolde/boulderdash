use crate::game::{display::action::Action, grid::Grid};

use super::super::enums::movement::Movement;

pub trait Fallable {
    fn fall(&self, grid: &Grid) -> Vec<Action>;
    fn is_falling(&self, grid: &Grid) -> Option<Movement>;
}