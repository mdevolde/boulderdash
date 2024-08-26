use std::any::Any;

use super::diamond::Diamond;
use super::enums::field::Field;
use super::grid::Grid;
use super::interfaces::entity::UnclonableEntity;
use super::interfaces::{collidable::Collidable, movable::Movable, renderable::Renderable};
use super::enums::movement::Movement;
use super::rock::Rock;

#[derive(Clone)]
pub struct Player {
    position: (i32, i32),
    doing: Movement,
    pushing: bool,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            position: (x, y),
            doing: Movement::Afk,
            pushing: false,
        }
    }

    pub fn push_rock(&mut self, grid: &mut Grid, rock: &mut Rock) {
        if self.pushing {
            let (rx, ry) = rock.get_position();
            if let Some(tile) = grid.get_nearest_tile(rx, ry, self.doing) {
                if tile.get_object_on().is_none() {
                    let (frx, fry) = self.doing.edit_position((rx, ry));
                    rock.move_to(rx, ry, frx, fry, grid);
                    self.move_to(self.position.0, self.position.1, rx, ry, grid);
                }
            }
        } else if self.doing == Movement::MoveLeft || self.doing == Movement::MoveRight {
            self.pushing = true;
        }
    }
}

impl Movable for Player {
    fn move_to(&mut self, ax: i32, ay: i32, nx: i32, ny: i32, grid: &mut Grid) {
        if let Some(actual_tile) = grid.get_mut_tile(ax, ay) {
            actual_tile.set_object_on(Field::Empty);
        }
        if let Some(new_tile) = grid.get_mut_tile(nx, ny) {
            self.position = (nx, ny);
            new_tile.set_object_on(Field::Entity(Box::new(self.clone())));
        }
    }
}

impl Collidable for Player {
    fn check_collision(&self, other: &dyn Collidable, grid: &Grid) -> bool {
        self.get_future_position(&grid) == other.get_position()
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
                                if self.pushing && direction == Movement::MoveLeft 
                                || direction == Movement::MoveRight { // TODO: reinitialize pushing when direction changes
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

impl UnclonableEntity for Player {
    fn get_type(&self) -> String {
        String::from("Player")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(&mut self, grid: &mut Grid) {
        let (x, y) = self.position;
        let (fx, fy) = self.get_future_position(&grid);
        if let Some(tile) = grid.get_tile(fx, fy) {
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Rock" {
                        let mut rock = entity.as_any().downcast_ref::<Rock>().unwrap().clone();
                        self.push_rock(grid, &mut rock);
                    } else {
                        self.move_to(x, y, fx, fy, grid)
                    }
                },
                Some(Field::Exit) => {
                    if grid.get_tiles_with_entity::<Diamond>().len() == 0 {
                        println!("You won!"); // Temporary implementation
                    }
                },
                Some(Field::Wall(_)) => (),
                _ => self.move_to(x, y, fx, fy, grid),
            };
        };
    }
}
