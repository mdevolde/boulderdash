use super::{grid::Grid, interfaces::renderable::Renderable};

pub struct Game {
    grid: Grid,
    score: i32,
    level: u32,
}

impl Game {
    pub fn new() -> Self {
        let level = 1;
        let level_file = Game::get_level_file(level as i32);
        let grid = Grid::new(&level_file);
        Game {
            grid,
            score: 0,
            level,
        }
    }

    pub fn get_level_file(level: i32) -> String {
        format!("./static/maps/level_{}.bbcff", level)
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
}

impl Renderable for Game {
    fn render(&self) {
        self.grid.render();
        // self.player.render(); Not neccessary because the player is rendered by the grid
    }
}
