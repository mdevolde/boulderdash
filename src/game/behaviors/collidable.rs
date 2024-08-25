pub trait Collidable {
    fn check_collision(&self, other: &dyn Collidable) -> bool;
    fn get_position(&self) -> (i32, i32);
}
