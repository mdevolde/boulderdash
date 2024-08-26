use std::any::Any;

use super::{diamond, enums::{field::Field, movement::Movement}, interfaces::{entity::Entity, renderable::Renderable}, rock::Rock, tile::Tile};

pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<Vec<Tile>>,
    player_position: (i32, i32),
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        let mut tiles = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(Tile::new(x, y, Field::Empty)); //TODO: Implement the level loading with file reading
            }
            tiles.push(row);
        }
        let player_position = (width / 2, height / 2); // Temporary implementation
        Grid {
            width,
            height,
            tiles,
            player_position,
        }
    }

    pub fn update(&mut self) {
        let mut actions = vec![];

        for rock in &self.get_tiles_with_entity::<Rock>() {
            actions.extend(rock.update(self));
        }
        
        let player_tile = self.get_tile(self.player_position.0, self.player_position.1).unwrap();
        actions.extend(player_tile.update(self));

        for diamond in &self.get_tiles_with_entity::<diamond::Diamond>() {
            actions.extend(diamond.update(self));
        }

        for action in actions {
            action.apply(self);
        }
    }

    pub fn get_mut_tiles(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.tiles
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
