use super::behaviors::{entity::Entity, renderable::Renderable};

pub struct Tile {
    x: i32,
    y: i32,
    entity: Option<Box<dyn Entity>>,
    visited: bool,
}

impl Renderable for Tile {
    fn render(&self) {
        match &self.entity {
            Some(entity) => entity.render(),
            None => println!("Empty tile at ({}, {})", self.x, self.y), // Temporary implementation
        }
    }
}
