use crate::game::display::action::Action;

pub trait Movable {
    fn move_to(&self, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action>;
}
