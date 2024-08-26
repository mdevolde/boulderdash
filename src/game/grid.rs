use super::{enums::movement::Movement, interfaces::renderable::Renderable, tile::Tile};

pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.iter().find(|tile| tile.get_position() == (x, y))
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
        for tile in &self.tiles {
            tile.render();
        }
    }
}
