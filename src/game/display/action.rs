use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use crate::game::enums::action_type::ActionType;

use super::{super::{enums::field::Field, grid::Grid, interfaces::renderable::Renderable}, zone::Zone};


#[derive(Debug)]
pub struct Action {
    coordinates: (i32, i32),
    field: Field,
    action_type: ActionType,
}

impl Action {
    pub fn new(coordinates: (i32, i32), field: Field, action_type: ActionType) -> Self {
        Action {
            coordinates,
            field,
            action_type
        }
    }

    pub fn apply(&self, grid: &mut Grid) {
        if let Some(tile) = grid.get_mut_tile(self.coordinates.0, self.coordinates.1) {
            tile.set_object_on(self.field.clone());
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.coordinates
    }

    pub fn get_action_type(&self) -> &ActionType {
        &self.action_type
    }
}

impl Renderable for Action {
    fn render(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        if let Some(tile) = grid.get_tile(self.coordinates.0, self.coordinates.1) {
            tile.render(grid, context, sprites, zone);
        }
    }
}
