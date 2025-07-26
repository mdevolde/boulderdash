use std::{any::Any, rc::Rc};

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::tile::Tile;

use super::{
    display::{action::Action, zone::Zone},
    enums::{action_type::ActionType, field::Field, movement::Movement},
    grid::Grid,
    interfaces::{
        collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable,
        renderable::Renderable,
    },
};

#[derive(Clone)]
pub struct Diamond {
    position: (i32, i32),
    falling_since: i32,
}

impl Diamond {
    pub fn new(x: i32, y: i32) -> Self {
        Diamond {
            position: (x, y),
            falling_since: 0,
        }
    }

    pub fn get_diamond_actions(grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        for diamond in grid.get_tiles_with_entity::<Diamond>() {
            actions.extend(diamond.update(grid));
        }
        actions
    }

    pub fn get_frame(&self, current_frame: i32) -> (f64, f64) {
        let frame_x = if (0..=7).contains(&current_frame) {
            current_frame as f64
        } else {
            0.0
        };

        (frame_x * 32.0, 10.0 * 32.0)
    }
}

impl Movable for Diamond {
    fn move_to(&self, grid: &Grid, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::new(
            (ax, ay),
            Field::Empty,
            ActionType::NoMoreEntityOnTile,
        ));
        let mut self_clone = self.clone();
        self_clone.position = (nx, ny);
        if let Some(tile) = grid.get_tile(nx, ny) {
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Player" {
                        actions.push(Action::new(
                            (nx, ny),
                            Field::Entity(Rc::new(self_clone)),
                            ActionType::KillPlayer,
                        ));
                    };
                }
                _ => actions.push(Action::new(
                    (nx, ny),
                    Field::Entity(Rc::new(self_clone)),
                    ActionType::FallableFall,
                )),
            };
        }
        actions
    }
}

impl Collidable for Diamond {
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

impl Renderable for Diamond {
    fn render(
        &self,
        grid: &Grid,
        context: &mut CanvasRenderingContext2d,
        sprites: &HtmlImageElement,
        zone: &Zone,
    ) {
        let (dx, dy) = zone.get_patched_position(self.position);
        let (diamond_x_in_sprite, diamond_y_in_sprite) = self.get_frame(grid.get_frame());

        let _ = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &sprites,
                diamond_x_in_sprite,
                diamond_y_in_sprite,
                32.0,
                32.0,
                dx,
                dy + 32.0,
                32.0,
                32.0,
            );
    }
}

impl Entity for Diamond {
    fn get_type(&self) -> String {
        String::from("Diamond")
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
        write!(f, "Diamond at {:?}", self.position)
    }
}

impl Fallable for Diamond {
    fn fall(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut self_clone = self.clone();
        if let Some(movement) = self.is_falling(grid) {
            let (x, y) = movement.edit_position(self.position);
            self_clone.falling_since += 1;
            actions.extend(self_clone.move_to(grid, self.position.0, self.position.1, x, y));
        } else if self.falling_since > 0 {
            self_clone.falling_since = 0;
            actions.push(Action::new(
                self.position,
                Field::Entity(Rc::new(self_clone)),
                ActionType::DiamondFallOnSomething,
            ));
        } else {
            actions.push(Action::new(
                self.position,
                Field::Entity(Rc::new(self_clone)),
                ActionType::FallableAFK,
            ));
        }
        actions
    }

    fn is_falling(&self, grid: &Grid) -> Option<Movement> {
        fn can_move_to(
            tile: Option<&Tile>,
            movement: Movement,
            falling_since: i32,
            grid: &Grid,
            depth: Option<u32>,
        ) -> Option<Movement> {
            let tile = tile?;
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Player"
                        && falling_since > 0
                        && movement == Movement::MoveDown
                    {
                        return Some(movement);
                    }
                }
                Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) => {
                    return None;
                }
                Some(Field::Empty) | None => {
                    if depth.map_or(true, |d| d > 0)
                        || can_move_to(
                            grid.get_nearest_tile(
                                tile.get_position().0,
                                tile.get_position().1,
                                Movement::MoveDown,
                            ),
                            movement,
                            falling_since,
                            grid,
                            Some(1),
                        )
                        .is_some()
                    {
                        return Some(movement);
                    }
                }
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
                    }
                    Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit)
                    | Some(Field::Empty) | None => continue,
                }
            }
        }
        false
    }
}
