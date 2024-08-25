use super::{actions::movement::Movement, behaviors::{collidable::Collidable, entity::{self, Entity}, fallable::Fallable, movable::Movable, renderable::Renderable}, grid::Grid};

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
                if let Some(entity) = tile.get_entity() {
                    return match entity.get_type().as_str() {
                        "Player" => direction.edit_position(self.position),
                        _ => self.position,
                    };
                } else {
                    return direction.edit_position(self.position);
                }
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

impl Fallable { // Temporary implementation
    fn fall(&mut self) {
        self.position.1 += 1;
    }

    fn is_falling(&self) -> Some<Movement> {
        Some(Movement::Down)
    }
}
