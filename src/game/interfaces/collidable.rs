use crate::game::grid::Grid;

pub trait Collidable {
    fn get_position(&self) -> (i32, i32);
    fn get_future_position(&self, grid: &Grid) -> (i32, i32);
}
