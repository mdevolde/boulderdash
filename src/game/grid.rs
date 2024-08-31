use std::{any::Any, rc::Rc};
use web_sys::{AudioBuffer, AudioContext, CanvasRenderingContext2d, HtmlImageElement};

use super::{diamond::{self, Diamond}, display::{action::Action, overlay::Overlay, scroller::Scroller, zone::Zone}, enums::{action_type::ActionType, field::Field, movement::Movement}, interfaces::{collidable::Collidable, entity::Entity, renderable::Renderable}, player::Player, rock::Rock, tile::Tile, wall::Wall};

#[derive(Debug)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
    player_position: (i32, i32),
    diamonds_number: i32,
    timer: f64,
    zones: Vec<Zone>,
    scroller: Option<Scroller>,
    frame: i32,
    last_frame_direction: Movement,
    last_frame_side_direction: Movement,
}

impl Grid {
    pub fn new(level_text: &str, canvas_sx: i32, canvas_sy: i32) -> Self {
        Grid::from_str(&level_text, canvas_sx, canvas_sy)
    }

    pub fn default() -> Self {
        Grid {
            tiles: vec![],
            player_position: (0, 0),
            diamonds_number: 0,
            timer: 0.0,
            zones: vec![],
            scroller: None,
            frame: 0,
            last_frame_direction: Movement::Afk,
            last_frame_side_direction: Movement::Afk,
        }
    }

    pub fn from_str(input: &str, canvas_sx: i32, canvas_sy: i32) -> Self {
        let mut lines = input.lines();

        let size_line = lines.next().expect("No size line found");
        let mut size_iter = size_line.split_whitespace();
        let height: i32 = size_iter.next().expect("Missing part in height").parse().expect("Could not parse height");
        let width: i32 = size_iter.next().expect("Missing part in width").parse().expect("Could not parse width");

        let player_line = lines.next().expect("No player line found");
        let mut player_iter = player_line.split_whitespace();
        let player_x: i32 = player_iter.next().expect("Missing part in x ").parse().expect("Could not parse player x");
        let player_y: i32 = player_iter.next().expect("Missing part in y").parse().expect("Could not parse player y");

        let diamonds_line = lines.next().expect("No diamonds line found");
        let diamonds_number: i32 = diamonds_line.parse().expect("Could not parse diamonds");

        lines.next();

        let mut tiles = Vec::new();
        for (y, line) in lines.enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let tile = Tile::new(x as i32, y as i32,
                    match ch {
                        'W' => Field::Wall(Wall::new(x as i32, y as i32)),
                        'r' => Field::Entity(Rc::new(Rock::new(x as i32, y as i32))),
                        'd' => Field::Entity(Rc::new(diamond::Diamond::new(x as i32, y as i32))),
                        '.' => Field::Dirt,
                        'P' => Field::Entity(Rc::new(Player::new(x as i32, y as i32))),
                        'X' => Field::Exit,
                        _ => Field::Empty,
                    }
                );
                row.push(tile);
            }
            tiles.push(row);
        }

        let zones = Zone::from_map(width, height, canvas_sx, canvas_sy);

        Grid {
            tiles,
            player_position: (player_x, player_y),
            diamonds_number,
            timer: 150.0,
            zones,
            scroller: None,
            frame: 0,
            last_frame_direction: Movement::Afk,
            last_frame_side_direction: Movement::Afk,
        }
    }

    pub fn update(&mut self, context: &mut CanvasRenderingContext2d, audio_context: &mut AudioContext, sprites: &HtmlImageElement, sounds: &Vec<AudioBuffer>) {
        if self.frame % 2 == 0 {
            let actions = Rock::get_rock_actions(self);
            self.apply_actions(actions, context, audio_context, sprites, sounds);
        }
        
        let zones = self.zones.clone();
        let zone = Zone::get_current_zone(self.player_position.0, self.player_position.1, &zones).expect("No zone found for player");

        self.scroll_if_needed(context, sprites, zone, &zones);

        if self.frame % 2 == 0 {
            let actions = Player::get_player_actions(self);
            self.set_last_frame_direction_afk_if_needed(&actions);
            self.apply_actions(actions, context, audio_context, sprites, sounds);
        }
    
        if let Some(player) = self.get_tiles_with_entity::<Player>().get(0) {
            self.player_position = player.get_position();
        }

        self.set_scroller_if_needed(zone);

        if self.frame % 2 == 0 {
            let actions = Diamond::get_diamond_actions(self);
            self.apply_actions(actions, context, audio_context, sprites, sounds);
        } else {
            self.render_diamonds_gif(context, sprites, zone);
        }

        let overlay = Overlay::new();
        overlay.render(self, context, sprites, zone);

        self.increment_frame();
        self.increment_timer();
    }

    pub fn apply_actions(&mut self, actions: Vec<Action>, context: &mut CanvasRenderingContext2d, audio_context: &mut AudioContext, sprites: &HtmlImageElement, sounds: &Vec<AudioBuffer>) {
        for action in actions {
            action.apply(self);
            if let Some(zone) = Zone::get_current_zone(self.player_position.0, self.player_position.1, &self.zones) {
                if zone.is_in_zone(action.get_position().0, action.get_position().1) && self.scroller.is_none() {
                    action.render(self, context, sprites, zone);
                    if self.frame % 2 == 0 {
                    }
                }
                self.play_action_sound(audio_context, action.get_action_type(), sounds);
            }
        }
    }

    pub fn play_action_sound(&self, audio_context: &AudioContext, action_type: &ActionType, sounds: &Vec<AudioBuffer>) {
        if let Some(audio_buffer) = action_type.get_linked_sound(&sounds) {
            let source = audio_context.create_buffer_source().unwrap();
            source.set_buffer(Some(audio_buffer));
            source.connect_with_audio_node(&audio_context.destination()).unwrap();
            source.set_loop(false);
            source.start().unwrap();
        }
    }

    pub fn scroll_if_needed(&mut self, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone, zones: &Vec<Zone>) {
        if let Some(scroller) = &mut self.scroller {
            if let Some(active_zone) = scroller.update() {
                active_zone.render(self, context, sprites, zone);
            } else {
                self.scroller = None;
                if let Some(new_zone) = Zone::get_current_zone(self.player_position.0, self.player_position.1, &zones) {
                    new_zone.render(self, context, sprites, zone);
                }
            }
        }
    }

    pub fn set_last_frame_direction_afk_if_needed(&mut self, actions: &Vec<Action>) {
        if let Some(action) = actions.get(0) {
            if action.get_position() == self.player_position && actions.len() == 1 {
                self.last_frame_direction = Movement::Afk;
            }
        }
    }

    pub fn set_scroller_if_needed(&mut self, zone: &Zone) {
        if let Some(current_zone) = Zone::get_current_zone(self.player_position.0, self.player_position.1, &self.zones) {
            if zone != current_zone {
                if let Some(scroller) = &self.scroller {
                    if let Some(active_zone) = scroller.get_active_zone() {
                        self.scroller = Some(Scroller::new(active_zone, *current_zone));
                    }
                } else {
                    self.scroller = Some(Scroller::new(*zone, *current_zone));
                }
            }
        }
    }

    pub fn render_diamonds_gif(&self, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        for diamond in self.get_tiles_with_entity::<diamond::Diamond>() {
            if self.scroller.is_none() && zone.is_in_zone(diamond.get_position().0, diamond.get_position().1) {
                diamond.render(self, context, sprites, zone);
            }
        }
    }

    pub fn increment_frame(&mut self) {
        if self.frame == 7 {
            self.frame = 0;
        } else {
            self.frame += 1;
        }
    }

    pub fn increment_timer(&mut self) {
        if self.timer > 0.0 {
            self.timer -= 0.05;
        } 
    }

    pub fn get_last_frame_direction(&self) -> Movement {
        self.last_frame_direction
    }

    pub fn get_last_frame_side_direction(&self) -> Movement {
        self.last_frame_side_direction
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get(y as usize).and_then(|row| row.get(x as usize))
    }

    pub fn get_mut_tile(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        self.tiles.get_mut(y as usize).and_then(|row| row.get_mut(x as usize))
    }

    pub fn get_nearest_tile(&self, x: i32, y: i32, direction: Movement) -> Option<&Tile> {
        match direction {
            Movement::Afk => None,
            other => {
                let coordinates = other.edit_position((x, y));
                self.get_tile(coordinates.0, coordinates.1)
            }
        }
    }

    pub fn get_tiles_with_entity<T: Entity + Any>(&self) -> Vec<&T> {
        let mut concerned_tiles = vec![];
        for row in &self.tiles {
            for tile in row {
                if let Some(Field::Entity(entity)) = tile.get_object_on() {
                    if let Some(entity) = entity.as_any().downcast_ref::<T>() {
                        concerned_tiles.push(entity);
                    }
                }
            }
        }
        concerned_tiles
    }

    pub fn get_frame(&self) -> i32 {
        self.frame
    }

    pub fn get_player_position(&self) -> (i32, i32) {
        self.player_position
    }

    pub fn get_diamonds_number(&self) -> i32 {
        self.diamonds_number
    }

    pub fn get_timer(&self) -> f64 {
        self.timer
    }

    pub fn render_player_zone(&mut self, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement) {
        if let Some(zone) = Zone::get_current_zone(self.player_position.0, self.player_position.1, &self.zones) {
            zone.render(self, context, sprites, &zone);
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.get_tiles_with_entity::<Player>().len() == 0
    }

    pub fn is_level_completed(&self) -> bool {
        for row in &self.tiles {
            for tile in row {
                if let Some(Field::Exit) = tile.get_object_on() {
                    return false;
                };
            };
        };
        true
    }

    pub fn set_player_doing(&mut self, movement: Movement) {
        if movement == Movement::MoveLeft || movement == Movement::MoveRight {
            self.last_frame_direction = movement;
            self.last_frame_side_direction = movement;
        } else {
            self.last_frame_direction = self.last_frame_side_direction;
        }
        
        let (x, y) = self.player_position;
        if let Some(player_tile) = self.get_tile(x, y) {
            if let Some(Field::Entity(entity)) = player_tile.get_object_on() {
                if let Some(player) = entity.as_any().downcast_ref::<Player>() {
                    let mut clone_player = player.clone();
                    clone_player.set_movement(movement);
                    let action: Action;
                
                    let field = Field::Entity(Rc::new(clone_player));
                    action = Action::new((x, y), field, ActionType::PlayerSetMovement);
                    
                    action.apply(self);
                }
            }
        }
    }
}
