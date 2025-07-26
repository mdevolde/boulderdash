use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{
    display::zone::Zone,
    grid::Grid,
    interfaces::{collidable::Collidable, renderable::Renderable},
};

#[derive(Clone, Debug)]
pub struct Wall {
    position: (i32, i32),
}

impl Wall {
    pub fn new(x: i32, y: i32) -> Self {
        Wall { position: (x, y) }
    }
}

impl Collidable for Wall {
    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, _: &Grid) -> (i32, i32) {
        self.position
    }
}

impl Renderable for Wall {
    fn render(
        &self,
        _: &Grid,
        context: &mut CanvasRenderingContext2d,
        sprites: &HtmlImageElement,
        zone: &Zone,
    ) {
        let (dx, dy) = zone.get_patched_position(self.position);
        let wall_x_in_sprite = (1 * 32) as f64;
        let wall_y_in_sprite = (6 * 32) as f64;
        let _ = context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &sprites,
                wall_x_in_sprite,
                wall_y_in_sprite,
                32.0,
                32.0,
                dx,
                dy + 32.0,
                32.0,
                32.0,
            );
    }
}
