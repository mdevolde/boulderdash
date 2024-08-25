use super::super::actions::movement::Movement;

pub trait Fallable {
    fn fall(&mut self);
    fn is_falling(&self) -> Some<Movement>;
}