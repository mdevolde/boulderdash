use super::behaviors::{collidable::Collidable, entity::Entity, movable::Movable, renderable::Renderable};
use super::actions::movement::Movement;

pub struct Player {
    position: (i32, i32),
    alive: bool,
    doing: Action,
}

impl Movable for Player {
    fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
}

impl Collidable for Player {
    fn check_collision(&self, other: &dyn Collidable) -> bool {
        self.position == other.get_position()
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}

impl Renderable for Player {
    fn render(&self) {
        println!("Player at {:?}", self.position); // Temporary implementation
    } 
}

impl Entity for Player {}
