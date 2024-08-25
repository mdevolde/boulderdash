use super::behaviors::{collidable::Collidable, entity::Entity, movable::Movable, renderable::Renderable};
use super::actions::movement::Movement;

pub struct Player {
    position: (i32, i32),
    alive: bool,
    doing: Action,
    pushing: bool,
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

    fn get_future_position(&self) -> (i32, i32) {
        if let Action::Move(direction) = self.doing {
            if let Some(tile) = grid.get_nearest_tile(self.position.0, self.position.1, direction) {
                if let Some(entity) = tile.get_entity() {
                    return match entity.get_type().as_str() {
                        "Rock" => {
                            if self.pushing {
                                direction.edit_position(self.position)
                            } else {
                                self.position
                            }
                        }
                        _ => direction.edit_position(self.position),
                    };
                } else {
                    return direction.edit_position(self.position);
                }
            }
        }
        self.position
    }
}

impl Renderable for Player {
    fn render(&self) {
        println!("Player at {:?}", self.position); // Temporary implementation
    } 
}

impl Entity for Player {
    fn get_type(&self) -> String {
        String::from("Player")
    }
}
