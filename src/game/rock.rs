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

impl Renderable for Rock {
    fn render(&self) {
        println!("Rock at {:?}", self.position); // Temporary implementation
    }
}

impl Entity for Rock {
    fn get_type(&self) -> String {
        String::from("Rock")
    }
}

impl Fallable for Rock { // Temporary implementation
    fn fall(&mut self) {
        self.position.1 += 1;
    }

    fn is_falling(&self) -> Some<Movement> {
        Some(Movement::Down)
    }
}
