use super::{grid::Grid, interfaces::{collidable::Collidable, renderable::Renderable}};

pub struct Wall {
    position: (i32, i32),
}

impl Collidable for Wall {
    fn check_collision(&self, _: &dyn Collidable, _: Grid) -> bool {
        false
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, _: &Grid) -> (i32, i32) {
        self.position
    }
}

impl Renderable for Wall {
    fn render(&self) {
        println!("Wall at {:?}", self.position); // Temporary implementation
    }
}
