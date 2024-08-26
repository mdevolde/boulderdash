use super::{enums::field::Field, grid::Grid};



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
        let tile = grid.get_mut_tile(self.coordinates.0, self.coordinates.1).unwrap();
        tile.set_object_on(self.field.clone());
    }
}