use std::any::Any;

use crate::game::tile::Tile;

use super::{enums::{field::Field, movement::Movement}, grid::Grid, interfaces::{collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable, renderable::Renderable}, player::Player};

#[derive(Clone)]
pub struct Rock {
    position: (i32, i32),
    falling_since: i32,
}

impl Rock {
    pub fn new(x: i32, y: i32) -> Self {
        Rock {
            position: (x, y),
            falling_since: 0,
        }
    }

    pub fn update(&mut self, grid: &mut Grid) {
        let (px, py) = grid.get_player_position();
        let player_tile = grid.get_tile(px, py).unwrap();
        if let Some(Field::Entity(entity)) = player_tile.get_object_on() {
            let player = entity.as_any().downcast_ref::<Player>().unwrap();
            if self.check_collision(player, grid) {
                //TODO: Implement the explosion rendering
            }
        }
        self.fall(grid);
    }

    fn is_fallable_near(&self, grid: &Grid) -> bool {
        let directions = vec![Movement::MoveDown, Movement::MoveLeft, Movement::MoveRight, Movement::MoveUp];
        for direction in directions {
            if let Some(tile) = grid.get_nearest_tile(self.position.0, self.position.1, direction) {
                match tile.get_object_on() {
                    Some(Field::Entity(entity)) => {
                        if entity.get_type().as_str() != "Player" {
                            return true;
                        }
                    },
                    Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) => return false,
                    Some(Field::Empty) | None => continue,
                }
            }
        }
        false
    }
}

impl Movable for Rock {
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

impl Collidable for Rock {
    fn check_collision(&self, other: &dyn Collidable, grid: &Grid) -> bool {
        self.get_future_position(&grid) == other.get_position()
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, grid: &Grid) -> (i32, i32) {
        if let Some(direction) = self.is_falling(grid) {
            direction.edit_position(self.position)
        } else {
            self.position
        }
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Fallable for Rock {
    fn fall(&mut self, grid: &mut Grid) {
        if let Some(movement) = self.is_falling(grid) {
            let (x, y) = movement.edit_position(self.position);
            self.move_to(
                self.position.0,
                self.position.1,
                x,
                y,
                grid,
            );
            self.falling_since += 1;
        } else {
            self.falling_since = 0;
        }
    }

    fn is_falling(&self, grid: &Grid) -> Option<Movement> {
        fn can_move_to(tile: Option<&Tile>, movement: Movement, falling_since: i32) -> Option<Movement> {
            match tile {
                Some(tile) => match tile.get_object_on() {
                    Some(Field::Entity(entity)) => {
                        if entity.get_type().as_str() == "Player" && falling_since > 0 && movement == Movement::MoveDown {
                            Some(movement)
                        } else {
                            None
                        }
                    },
                    Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) => None,
                    Some(Field::Empty) | None => Some(movement),
                },
                None => None,
            }
        }

        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveDown),
            Movement::MoveDown,
            self.falling_since,
        ) {
            return Some(movement);
        }
    
        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveLeft),
            Movement::MoveLeft,
            self.falling_since,
        ) {
            if self.is_fallable_near(grid) {
                return Some(movement);
            }
        }
    
        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveRight),
            Movement::MoveRight,
            self.falling_since,
        ) {
            if self.is_fallable_near(grid) {
                return Some(movement);
            }
        }
    
        None
    }
}
