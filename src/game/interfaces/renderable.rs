use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::{grid::Grid, display::zone::Zone};

pub trait Renderable {
    fn render(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone);
}