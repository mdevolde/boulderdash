use crate::game::{display::action::Action, grid::Grid};

pub trait Movable {
    fn move_to(&self, grid: &Grid, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action>;
}
