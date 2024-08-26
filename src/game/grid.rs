use super::{enums::movement::Movement, interfaces::renderable::Renderable, tile::Tile};

pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
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
