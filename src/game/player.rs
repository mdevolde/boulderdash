use super::enums::field::Field;
use super::grid::Grid;
use super::interfaces::{collidable::Collidable, entity::Entity, movable::Movable, renderable::Renderable};
use super::enums::movement::Movement;

pub struct Player {
    position: (i32, i32),
    alive: bool,
    doing: Movement,
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

    fn get_future_position(&self, grid: &Grid) -> (i32, i32) {
        match self.doing {
            Movement::Afk => self.position,
            direction => {
                if let Some(tile) = grid.get_nearest_tile(self.position.0, self.position.1, direction) {
                    match tile.get_object_on() {
                        Some(Field::Entity(entity)) => match entity.get_type().as_str() {
                            "Rock" => {
                                if self.pushing {
                                    return direction.edit_position(self.position);
                                } else {
                                    return self.position;
                                }
                            }
                            _ => return direction.edit_position(self.position),
                        },
                        Some(Field::Wall(_)) => return self.position,
                        Some(Field::Exit) | Some(Field::Empty) | Some(Field::Dirt) | None => {
                            return direction.edit_position(self.position)
                        }
                    }
                }
                self.position
            }
        }
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
