use web_sys::{CanvasRenderingContext2d, HtmlImageElement};
use super::{display::action::Action, enums::field::Field, grid::Grid, interfaces::renderable::Renderable, display::zone::Zone};

#[derive(Debug)]
pub struct Tile {
    x: i32,
    y: i32,
    field: Field,
}

impl Tile {
    pub fn new(x: i32, y: i32, field: Field) -> Self {
        Tile {
            x,
            y,
            field,
        }
    }

    pub fn update(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = vec![];
        match self.field {
            Field::Entity(ref entity) => actions.extend(entity.update(grid)),
            _ => (),
        }
        actions
    }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn get_object_on(&self) -> Option<&Field> {
        match &self.field {
            Field::Entity(_) | Field::Wall(_) | Field::Dirt | Field::Exit => Some(&self.field),
            Field::Empty => None,
        }
    }

    pub fn set_object_on(&mut self, field: Field) {
        self.field = field;
    }

    pub fn render_non_obj(&self, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone, sx: f64, sy: f64) {
        let (dx, dy) = zone.get_patched_position((self.x, self.y));
        let _ = context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &sprites, 
            sx, sy, 
            32.0, 32.0, 
            dx, dy+32.0, 
            32.0, 32.0,
        );
    }
}

impl Renderable for Tile {
    fn render(&self, grid: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        match &self.get_object_on() {
            Some(Field::Entity(entity)) => entity.render(grid, context, sprites, zone),
            Some(Field::Wall(wall)) => wall.render(grid, context, sprites, zone),
            Some(Field::Dirt) => self.render_non_obj(context, sprites, zone, (1 * 32) as f64, (7 * 32) as f64),
            
            Some(Field::Exit) => self.render_non_obj(context, sprites, zone, (2 * 32) as f64, (6 * 32) as f64),
            Some(Field::Empty) | None => self.render_non_obj(context, sprites, zone, (0 * 32) as f64, (6 * 32) as f64),
        };
    }
}
