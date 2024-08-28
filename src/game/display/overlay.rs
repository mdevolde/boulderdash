use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::{diamond::Diamond, grid::Grid, interfaces::renderable::Renderable};

use super::zone::Zone;

pub struct Overlay {}

impl Overlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_sentence_positions(&self, sentence: &str, yellow: bool) -> Vec<(f64, f64)> {
        let mut positions = Vec::new();
        let mut x = if yellow { 9.0 } else { 8.0 };
        let mut y;
    
        for c in sentence.chars() {
            match c {
                'A'..='Z' => {
                    y = 33.0 + (c as u8 - b'A') as f64;
                    positions.push((x, y));
                }
                '0'..='9' => {
                    y = 16.0 + (c as u8 - b'0') as f64;
                    positions.push((x, y));
                }
                '^' => {
                    let y = 4.0;
                    positions.push((x, y));
                }
                ' ' => {
                    x -= 1.0;
                }
                _ => {}
            }
        }
    
        positions
    }

    pub fn render_diamonds_number(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement) {
        let mut positions = vec![];
        positions.extend(self.get_sentence_positions(&grid.get_diamonds_number().to_string(), true));
        positions.extend(self.get_sentence_positions("^", false));
        positions.extend(self.get_sentence_positions(&grid.get_diamonds_number().to_string(), false));
        for (i, (x, y)) in positions.iter().enumerate() {
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                sprites,
                x * 32.0,
                (y * 16.0) + 1.0,
                32.0,
                16.0,
                32.0 * i as f64,
                0.0,
                32.0,
                32.0,
            )
            .unwrap();
        }
    }

    pub fn render_diamonds_claimed(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement) {
        let mut diamonds_claimed =  (grid.get_diamonds_number() - grid.get_tiles_with_entity::<Diamond>().len() as i32).to_string();
        if diamonds_claimed.len() == 1 {
            diamonds_claimed = format!("0{}", diamonds_claimed);
        }

        let positions = self.get_sentence_positions(&diamonds_claimed, true);
        for (i, (x, y)) in positions.iter().enumerate() {
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                sprites,
                x * 32.0,
                (y * 16.0) + 1.0,
                32.0,
                16.0,
                (32.0 * i as f64) + (10.0 * 32.0),
                0.0,
                32.0,
                32.0,
            )
            .unwrap();
        }
    }

    pub fn render_timer(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement) {
        let mut timer = (grid.get_timer() as u64).to_string();
        if timer.len() == 1 {
            timer = format!("00{}", timer);
        } else if timer.len() == 2 {
            timer = format!("0{}", timer);
        }

        let positions = self.get_sentence_positions(&timer, false);
        for (i, (x, y)) in positions.iter().enumerate() {
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                sprites,
                x * 32.0,
                (y * 16.0) + 1.0,
                32.0,
                16.0,
                (32.0 * i as f64) + (16.0 * 32.0),
                0.0,
                32.0,
                32.0,
            )
            .unwrap();
        }
    }

    pub fn render_score(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement) {
        let mut score =  ((grid.get_diamonds_number() - grid.get_tiles_with_entity::<Diamond>().len() as i32)*10).to_string();
        if score.len() == 1 {
            score = format!("00000{}", score);
        } else if score.len() == 2 {
            score = format!("0000{}", score);
        } else if score.len() == 3 {
            score = format!("000{}", score);
        } else if score.len() == 4 {
            score = format!("00{}", score);
        } else if score.len() == 5 {
            score = format!("0{}", score);
        }

        let positions = self.get_sentence_positions(&score, false);
        for (i, (x, y)) in positions.iter().enumerate() {
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                sprites,
                x * 32.0,
                (y * 16.0) + 1.0,
                32.0,
                16.0,
                (32.0 * i as f64) + (24.0 * 32.0),
                0.0,
                32.0,
                32.0,
            )
            .unwrap();
        }
    }
}

impl Renderable for Overlay {
    fn render(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, _: &Zone) {
        self.render_diamonds_number(grid, context, sprites);
        self.render_diamonds_claimed(grid, context, sprites);
        self.render_timer(grid, context, sprites);
        self.render_score(grid, context, sprites);
    }
}
