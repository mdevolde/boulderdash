use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::{display::zone::Zone, grid::Grid};

pub trait Renderable {
    fn render(
        &self,
        grid: &Grid,
        context: &mut CanvasRenderingContext2d,
        sprites: &HtmlImageElement,
        zone: &Zone,
    );
}
