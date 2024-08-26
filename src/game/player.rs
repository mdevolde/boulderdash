use std::any::Any;
use std::rc::Rc;

use super::action::Action;
use super::diamond::Diamond;
use super::enums::field::Field;
use super::grid::Grid;
use super::interfaces::entity::Entity;
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

    pub fn push_rock(&self, grid: &Grid, rock: &Rock) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.pushing {
            let (rx, ry) = rock.get_position();
            if let Some(tile) = grid.get_nearest_tile(rx, ry, self.doing) {
                if tile.get_object_on().is_none() {
                    let (frx, fry) = self.doing.edit_position((rx, ry));
                    actions.extend(rock.move_to(rx, ry, frx, fry));
                    actions.extend(self.move_to(self.position.0, self.position.1, rx, ry));
                };
            };
        } else if self.doing == Movement::MoveLeft || self.doing == Movement::MoveRight {
            let mut self_copy = self.clone();
            self_copy.pushing = true;
            actions.push(Action::new(self.position, Field::Entity(Rc::new(self_copy))));
        };
        actions
    }
}

impl Movable for Player {
    fn move_to(&self, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::new((ax, ay), Field::Empty));
        actions.push(Action::new((nx, ny), Field::Entity(Rc::new(self.clone()))));
        actions
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

impl Entity for Player {
    fn get_type(&self) -> String {
        String::from("Player")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        let (x, y) = self.position;
        let (fx, fy) = self.get_future_position(&grid);
        if let Some(tile) = grid.get_tile(fx, fy) {
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Rock" {
                        let rock = entity.as_any().downcast_ref::<Rock>().unwrap().clone();
                        actions.extend(self.push_rock(grid, &rock));
                    } else {
                        actions.extend(self.move_to(x, y, fx, fy));
                    };
                },
                Some(Field::Exit) => {
                    if grid.get_tiles_with_entity::<Diamond>().len() == 0 {
                        println!("You won!"); // Temporary implementation
                    };
                },
                Some(Field::Wall(_)) => (),
                _ => actions.extend(self.move_to(x, y, fx, fy)),
            };
        };
        actions
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player at {:?}", self.position)
    }
}
