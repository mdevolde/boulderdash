use super::behaviors::{collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable, renderable::Renderable};

pub struct Rock {
    position: (i32, i32),
    falling: bool,
}

impl Movable for Rock {
    fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
}

impl Collidable for Rock {
    fn check_collision(&self, other: &dyn Collidable) -> bool {
        self.position == other.get_position()
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }
}

impl Renderable for Rock {
    fn render(&self) {
        println!("Rock at {:?}", self.position); // Temporary implementation
    }
}

impl Entity for Rock {}

impl Fallable for Rock { // Temporary implementation
    fn fall(&mut self) {
        self.position.1 += 1;
    }

    fn is_falling(&self) -> Some<Movement> {
        Some(Movement::Down)
    }
}
