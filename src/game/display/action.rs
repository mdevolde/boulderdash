use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use super::{super::{enums::field::Field, grid::Grid, interfaces::renderable::Renderable}, zone::Zone};


#[derive(Debug)]
pub struct Action {
    coordinates: (i32, i32),
    field: Field,
}

impl Action {
    pub fn new(coordinates: (i32, i32), field: Field) -> Self {
        Action {
            coordinates,
            field,
        }
    }

    pub fn apply(&self, grid: &mut Grid) {
        if let Some(tile) = grid.get_mut_tile(self.coordinates.0, self.coordinates.1) {
            tile.set_object_on(self.field.clone());
        }
    }
}

impl Renderable for Action {
    fn render(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        if let Some(tile) = grid.get_tile(self.coordinates.0, self.coordinates.1) {
            tile.render(grid, context, sprites, zone);
        }
    }
}
