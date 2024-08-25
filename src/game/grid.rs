use super::{behaviors::renderable::Renderable, tile::Tile};

pub struct Grid {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Renderable for Grid {
    fn render(&self) {
        for tile in &self.tiles {
            tile.render();
        }
    }
}
