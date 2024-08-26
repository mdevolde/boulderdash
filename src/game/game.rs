use super::{interfaces::renderable::Renderable, grid::Grid, player::Player};

pub struct Game {
    grid: Grid,
    player: Player,
    score: i32,
    level: u32,
}

impl Renderable for Game {
    fn render(&self) {
        self.grid.render();
        // self.player.render(); Not neccessary because the player is rendered by the grid
    }
}
