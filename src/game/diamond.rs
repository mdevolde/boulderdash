use super::{enums::{field::Field, movement::Movement}, grid::Grid, interfaces::{collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable, renderable::Renderable}};

pub struct Diamond {
    position: (i32, i32),
    collected: bool,
}

impl Movable for Diamond {
    fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }
}

impl Collidable for Diamond {
    fn check_collision(&self, other: &dyn Collidable) -> bool {
        self.position == other.get_position()
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, grid: &Grid) -> (i32, i32) {
        if let Some(direction) = self.is_falling() {
            if let Some(tile) = grid.get_nearest_tile(self.position.0, self.position.1, direction) {
                match tile.get_object_on() {
                    Some(Field::Entity(entity)) => {
                        match entity.get_type().as_str() {
                            "Player" => direction.edit_position(self.position),
                            _ => self.position,
                        }
                    },
                    Some(Field::Wall(_)) | Some(Field::Exit) => self.position,
                    Some(Field::Empty) | None => direction.edit_position(self.position),
                };
            }
        }
        self.position
    }
}

impl Renderable for Diamond {
    fn render(&self) {
        println!("Diamond at {:?}", self.position); // Temporary implementation
    }
}

impl Entity for Diamond {
    fn get_type(&self) -> String {
        String::from("Diamond")
    }
}

impl Fallable for Diamond { // Temporary implementation
    fn fall(&mut self) {
        self.position.1 += 1;
    }

    fn is_falling(&self) -> Option<Movement> {
        Some(Movement::MoveDown)
    }
}
