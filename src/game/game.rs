use super::{grid::Grid, interfaces::renderable::Renderable};

pub struct Game {
    grid: Grid,
    score: i32,
    level: u32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        let grid = Grid::new(width, height);
        Game {
            grid,
            score: 0,
            level: 1,
        }
    }
}

impl Renderable for Game {
    fn render(&self) {
        self.grid.render();
        // self.player.render(); Not neccessary because the player is rendered by the grid
    }
}
