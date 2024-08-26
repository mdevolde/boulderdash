use std::{any::Any, rc::Rc};

use super::{diamond, enums::{field::Field, movement::Movement}, interfaces::{entity::Entity, renderable::Renderable}, player::Player, rock::Rock, tile::Tile, wall::Wall};

#[derive(Debug)]
pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<Vec<Tile>>,
    player_position: (i32, i32),
}

impl Grid {
    pub fn new(file_path: &str) -> Self {
        let file = std::fs::read_to_string(file_path).expect("Could not read file");
        Grid::from_str(&file)
    }

    pub fn from_str(input: &str) -> Self {
        let mut lines = input.lines();

        let size_line = lines.next().unwrap();
        let mut size_iter = size_line.split_whitespace();
        let height: i32 = size_iter.next().unwrap().parse().unwrap();
        let width: i32 = size_iter.next().unwrap().parse().unwrap();

        let player_line = lines.next().unwrap();
        let mut player_iter = player_line.split_whitespace();
        let player_x: i32 = player_iter.next().unwrap().parse().unwrap();
        let player_y: i32 = player_iter.next().unwrap().parse().unwrap();

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
                        '.' => Field::Empty,
                        'P' => Field::Entity(Rc::new(Player::new(x as i32, y as i32))),
                        'X' => Field::Exit,
                        _ => Field::Empty,
                    }
                );
                row.push(tile);
            }
            tiles.push(row);
        }

        Grid {
            width,
            height,
            tiles,
            player_position: (player_x, player_y),
        }
    }

    pub fn update(&mut self) {
        let mut actions = vec![];

        for rock in self.get_tiles_with_entity::<Rock>() {
            actions.extend(rock.update(self));
        }
        
        let player_tile = self.get_tile(self.player_position.0, self.player_position.1).unwrap();
        actions.extend(player_tile.update(self));

        for diamond in self.get_tiles_with_entity::<diamond::Diamond>() {
            actions.extend(diamond.update(self));
        }

        for action in actions {
            action.apply(self);
        }
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

    pub fn get_player_position(&self) -> (i32, i32) {
        self.player_position
    }
}

impl Renderable for Grid {
    fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_tile(x, y) {
                    Some(tile) => tile.render(),
                    None => println!("No tile at ({}, {})", x, y), // Temporary implementation
                }
            }
        }
    }
}
