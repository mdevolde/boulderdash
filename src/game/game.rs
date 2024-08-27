use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::grid::Grid;

pub struct Game {
    grid: Grid,
    score: i32,
    level: u32,
    context: CanvasRenderingContext2d,
    sprites: HtmlImageElement,
    levels: Vec<String>,
}

impl Game {
    pub fn new(context: CanvasRenderingContext2d, sprites: HtmlImageElement, levels: Vec<String>) -> Self {
        let level = 1;
        let level_text = Game::get_level_text(level, &levels);
        let canvas_width = context.canvas().expect("No canvas found").width();
        let canvas_height = context.canvas().expect("No canvas found").height();
        let grid = Grid::new(&level_text, canvas_width as i32, canvas_height as i32);
        Game {
            grid,
            score: 0,
            level: level as u32,
            context,
            sprites,
            levels,
        }
    }

    pub fn get_level_text(level: usize, levels: &Vec<String>) -> String {
        levels[level-1].clone()
    }

    pub fn update(&mut self) {
        self.grid.render_player_zone(&mut self.context, &self.sprites)
    }
}
