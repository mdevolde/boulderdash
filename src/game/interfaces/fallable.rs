use super::super::enums::movement::Movement;

pub trait Fallable {
    fn fall(&mut self);
    fn is_falling(&self) -> Option<Movement>;
}