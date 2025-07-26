use std::any::Any;
use std::rc::Rc;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::diamond::Diamond;
use super::display::action::Action;
use super::display::zone::Zone;
use super::enums::action_type::ActionType;
use super::enums::field::Field;
use super::enums::movement::Movement;
use super::grid::Grid;
use super::interfaces::entity::Entity;
use super::interfaces::{collidable::Collidable, movable::Movable, renderable::Renderable};
use super::rock::Rock;

#[derive(Clone, Debug)]
pub struct Player {
    position: (i32, i32),
    doing: Movement,
    pushing: Option<Movement>,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            position: (x, y),
            doing: Movement::Afk,
            pushing: None,
        }
    }

    pub fn get_player_actions(grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        if let Some(player_tile) =
            grid.get_tile(grid.get_player_position().0, grid.get_player_position().1)
        {
            actions.extend(player_tile.update(grid));
        }
        actions
    }

    pub fn set_movement(&mut self, movement: Movement) {
        self.doing = movement;
    }

    pub fn cancel_push(&self, afk: bool) -> Action {
        let mut self_clone = self.clone();
        self_clone.pushing = None;
        if afk {
            self_clone.doing = Movement::Afk;
        };
        Action::new(
            self.position,
            Field::Entity(Rc::new(self_clone)),
            ActionType::PlayerCancelPush,
        )
    }

    pub fn push_rock(&self, grid: &Grid, rock: &Rock) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.pushing.is_some() && self.doing == self.pushing.unwrap() {
            let (rx, ry) = rock.get_position();
            if let Some(tile) = grid.get_nearest_tile(rx, ry, self.doing) {
                if tile.get_object_on().is_none() {
                    let (frx, fry) = self.doing.edit_position((rx, ry));
                    actions.extend(rock.move_to(grid, rx, ry, frx, fry));
                    actions.extend(self.move_to(grid, self.position.0, self.position.1, rx, ry));
                };
            };
        } else if self.doing == Movement::MoveLeft || self.doing == Movement::MoveRight {
            let mut self_copy = self.clone();
            self_copy.pushing = Some(self.doing);
            self_copy.doing = Movement::Afk;
            actions.push(Action::new(
                self.position,
                Field::Entity(Rc::new(self_copy)),
                ActionType::PlayerSetPush,
            ));
        };
        actions
    }

    pub fn get_frame(&self, current_frame: i32, action: Movement) -> (f64, f64) {
        let row = match action {
            Movement::MoveLeft => 4.0,
            Movement::MoveRight => 5.0,
            _ => 3.0,
        };

        let column = if (0..=7).contains(&current_frame) {
            current_frame as f64
        } else {
            0.0
        };

        (column * 32.0, row * 32.0)
    }
}

impl Movable for Player {
    fn move_to(&self, grid: &Grid, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::new(
            (ax, ay),
            Field::Empty,
            ActionType::NoMoreEntityOnTile,
        ));
        let mut self_clone = self.clone();
        self_clone.doing = Movement::Afk;
        self_clone.position = (nx, ny);
        self_clone.pushing = None;
        if let Some(tile) = grid.get_tile(nx, ny) {
            match tile.get_object_on() {
                Some(Field::Entity(entity)) => {
                    if entity.get_type().as_str() == "Diamond" {
                        actions.push(Action::new(
                            (nx, ny),
                            Field::Entity(Rc::new(self_clone)),
                            ActionType::ClaimDiamond,
                        ));
                    } else {
                        actions.push(Action::new(
                            (nx, ny),
                            Field::Entity(Rc::new(self_clone)),
                            ActionType::PlayerMove,
                        ));
                    };
                }
                _ => actions.push(Action::new(
                    (nx, ny),
                    Field::Entity(Rc::new(self_clone)),
                    ActionType::PlayerMove,
                )),
            };
        }
        actions
    }
}

impl Collidable for Player {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, grid: &Grid) -> (i32, i32) {
        match self.doing {
            Movement::Afk => self.position,
            direction => {
                if let Some(tile) =
                    grid.get_nearest_tile(self.position.0, self.position.1, direction)
                {
                    match tile.get_object_on() {
                        Some(Field::Entity(entity)) => match entity.get_type().as_str() {
                            "Rock" => match direction {
                                Movement::MoveLeft | Movement::MoveRight => {
                                    return direction.edit_position(self.position);
                                }
                                _ => return self.position,
                            },
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
    fn render(
        &self,
        grid: &Grid,
        context: &mut CanvasRenderingContext2d,
        sprites: &HtmlImageElement,
        zone: &Zone,
    ) {
        let (dx, dy) = zone.get_patched_position(self.position);

        let direction: Movement;
        if grid.get_last_frame_direction() == Movement::Afk {
            direction = Movement::Afk;
        } else {
            direction = grid.get_last_frame_side_direction();
        }

        let (sx, sy) = self.get_frame(grid.get_frame(), direction);
        let _ = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &sprites,
                sx,
                sy,
                32.0,
                32.0,
                dx,
                dy + 32.0,
                32.0,
                32.0,
            );
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
                        let rock = entity
                            .as_any()
                            .downcast_ref::<Rock>()
                            .expect("Downcast failed for a Rock")
                            .clone();
                        actions.extend(self.push_rock(grid, &rock));
                    } else if entity.get_type().as_str() == "Diamond" {
                        actions.extend(self.move_to(grid, x, y, fx, fy));
                    } else {
                        actions.push(self.cancel_push(true));
                    };
                }
                Some(Field::Exit) => {
                    if grid.get_tiles_with_entity::<Diamond>().len() == 0 {
                        actions.extend(self.move_to(grid, x, y, fx, fy));
                    };
                }
                Some(Field::Wall(_)) => actions.push(self.cancel_push(false)),
                _ => actions.extend(self.move_to(grid, x, y, fx, fy)),
            };
        };
        actions
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player at {:?}", self.position)
    }
}
