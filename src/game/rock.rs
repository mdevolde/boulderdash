use std::{any::Any, rc::Rc};

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::tile::Tile;

use super::{display::action::Action, enums::{field::Field, movement::Movement}, grid::Grid, interfaces::{collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable, renderable::Renderable}, display::zone::Zone};

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

    pub fn get_rock_actions(grid: &Grid) -> Vec<Action> {
        let mut actions = vec![];
        for rock in grid.get_tiles_with_entity::<Rock>() {
            actions.extend(rock.update(grid));
        };
        actions
    }
}

impl Movable for Rock {
    fn move_to(&self, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::new((ax, ay), Field::Empty));
        let mut self_clone = self.clone();
        self_clone.position = (nx, ny);
        actions.push(Action::new((nx, ny), Field::Entity(Rc::new(self_clone))));
        actions
    }
}

impl Collidable for Rock {
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
    fn render(&self, _: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        let (dx, dy) = zone.get_patched_position(self.position);
        let rock_x_in_sprite = 0.0;
        let rock_y_in_sprite = (7 * 32) as f64;
        let _ = context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &sprites, 
            rock_x_in_sprite, rock_y_in_sprite, 
            32.0, 32.0, 
            dx, dy+32.0, 
            32.0, 32.0,
        );
    }
}

impl Entity for Rock {
    fn get_type(&self) -> String {
        String::from("Rock")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.extend(self.fall(grid));
        actions
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rock at {:?}", self.position)
    }
}

impl Fallable for Rock {
    fn fall(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut self_clone = self.clone();
        if let Some(movement) = self.is_falling(grid) {
            let (x, y) = movement.edit_position(self.position);
            self_clone.falling_since += 1;
            actions.extend(self_clone.move_to(self.position.0, self.position.1, x, y));
        } else {
            self_clone.falling_since = 0;
            actions.push(Action::new(self.position, Field::Entity(Rc::new(self_clone))));
        }
        actions
    }

    fn is_falling(&self, grid: &Grid) -> Option<Movement> {
        fn can_move_to(tile: Option<&Tile>, movement: Movement, falling_since: i32, grid: &Grid, depth: Option<u32>) -> Option<Movement> {
            let tile = tile?;
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Player" && falling_since > 0 && movement == Movement::MoveDown {
                        return Some(movement);
                    }
                },
                Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) => {
                    return None;
                },
                Some(Field::Empty) | None => {
                    if depth.map_or(true, |d| d > 0) || 
                        can_move_to(
                            grid.get_nearest_tile(tile.get_position().0, tile.get_position().1, Movement::MoveDown),
                            movement, 
                            falling_since, 
                            grid, 
                            Some(1)
                        ).is_some() {
                        return Some(movement);
                    }
                },
            }
            None
        }

        let movements = [
        (Movement::MoveDown, None),
        (Movement::MoveLeft, Some(0)),
        (Movement::MoveRight, Some(0)),
        ];
        for &(movement, depth) in &movements {
            if let Some(movement) = can_move_to(
                grid.get_nearest_tile(self.position.0, self.position.1, movement),
                movement,
                self.falling_since,
                grid,
                depth,
            ) {
                if movement == Movement::MoveDown || self.is_fallable_near(grid) {
                    return Some(movement);
                }
            }
        }
        None
    }

    fn is_fallable_near(&self, grid: &Grid) -> bool {
        let directions = vec![Movement::MoveDown, Movement::MoveLeft, Movement::MoveRight];
        for direction in directions {
            if let Some(tile) = grid.get_nearest_tile(self.position.0, self.position.1, direction) {
                match tile.get_object_on() {
                    Some(Field::Entity(entity)) => {
                        if entity.get_type().as_str() != "Player" {
                            return true;
                        }
                    },
                    Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) | Some(Field::Empty) | None => continue,
                }
            }
        }
        false
    }
}
