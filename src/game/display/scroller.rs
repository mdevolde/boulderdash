use crate::game::interfaces::renderable::Renderable;

use super::zone::Zone;

pub struct ZoneScroller {
    current_zone: Zone,
    target_zone: Option<Zone>,
    scrolling: bool,
    scroll_speed: f64,
    current_offset_x: f64,
    current_offset_y: f64,
}

impl ZoneScroller {
    pub fn new(start_zone: Zone) -> Self {
        ZoneScroller {
            current_zone: start_zone,
            target_zone: None,
            scrolling: false,
            scroll_speed: 32.0, // Pixels per frame
            current_offset_x: 0.0,
            current_offset_y: 0.0,
        }
    }

    pub fn scroll_to_zone(&mut self, target: Zone) {
        self.target_zone = Some(target);
        self.scrolling = true;
        self.current_offset_x = 0.0;
        self.current_offset_y = 0.0;
    }

    pub fn update(&mut self) {
        if self.scrolling {
            if let Some(target) = self.target_zone {
                let diff_x = (target.get_sx() - self.current_zone.get_sx()) as f64 - self.current_offset_x;
                let diff_y = (target.get_sy() - self.current_zone.get_sy()) as f64 - self.current_offset_y;

                let distance = (diff_x.powi(2) + diff_y.powi(2)).sqrt();

                if distance < self.scroll_speed {
                    self.current_zone = target;
                    self.target_zone = None;
                    self.scrolling = false;
                } else {
                    self.current_offset_x += (diff_x / distance) * self.scroll_speed;
                    self.current_offset_y += (diff_y / distance) * self.scroll_speed;
                }
            }
        }
    }
}

impl Renderable for ZoneScroller {
    fn render(&self, grid: &crate::game::grid::Grid, context: &mut web_sys::CanvasRenderingContext2d, sprites: &web_sys::HtmlImageElement, _: &Zone) {
        let offset_x = self.current_offset_x;
        let offset_y = self.current_offset_y;

        for y in self.current_zone.get_sy()..self.current_zone.get_ey() {
            for x in self.current_zone.get_sx()..self.current_zone.get_ex() {
                let tile = grid.get_tile(x, y).expect("Tile not found");
                context.save();
                context.translate(-offset_x, -offset_y).unwrap();
                tile.render(grid, context, sprites, &self.current_zone);
                context.restore();
            }
        };
    }
}

