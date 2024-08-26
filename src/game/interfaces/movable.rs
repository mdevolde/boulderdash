use crate::game::grid::Grid;

pub trait Movable {
    fn move_to(&mut self, ax: i32, ay: i32, nx: i32, ny: i32, grid: &mut Grid);
}
