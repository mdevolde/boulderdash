use crate::game::interfaces::renderable::Renderable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zone {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
}

impl Zone {
    pub fn from_map(width: i32, height: i32, mut canvas_width: i32, mut canvas_height: i32) -> Vec<Zone> {
        let mut zones = Vec::new();

        canvas_width /= 32;
        canvas_height /= 32;

        let overlap_x = width - canvas_width;
        let overlap_y = height - canvas_height;
    
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let mut start_x = x;
                let end_x = (x + canvas_width).min(width);
                let mut start_y = y;
                let end_y = (y + canvas_height).min(height);

                if (end_x - start_x) < canvas_width {
                    start_x -= (start_x - overlap_x).max(0);
                }

                if (end_y - start_y) < canvas_height {
                    start_y -= (start_y - overlap_y).max(0);
                }
                
                zones.push(Zone {
                    start_x,
                    end_x,
                    start_y,
                    end_y,
                });

                if end_x == width {
                    break;
                }
                x += canvas_width - overlap_x;
            }
            let lz = zones.pop().unwrap();
            let end_y = lz.end_y;
            zones.push(lz);
            if end_y == height {
                break;
            }
            y += canvas_height - overlap_y;
        }
    
        zones
    }

    pub fn get_current_zone(player_x: i32, player_y: i32, zones: &[Zone]) -> Option<&Zone> {
        let mut possible_zones: Vec<&Zone> = zones.iter()
            .filter(|zone| {
                player_x >= zone.start_x && player_x < zone.end_x &&
                player_y >= zone.start_y && player_y < zone.end_y
            })
            .collect();
    
        if possible_zones.len() == 1 {
            return possible_zones.pop();
        }
    
        possible_zones.sort_by_key(|zone| {
            let distance_x = (player_x - zone.start_x).abs().min((zone.end_x - player_x).abs());
            let distance_y = (player_y - zone.start_y).abs().min((zone.end_y - player_y).abs());
            -(distance_x + distance_y)
        });
    
        possible_zones.first().copied()
    }

    pub fn get_patched_position(&self, (x, y): (i32, i32)) -> (f64, f64) {
        let x = x - self.start_x;
        let y = y - self.start_y;
        ((x * 32) as f64, (y*32) as f64)
    }

    pub fn get_sx(&self) -> i32 {
        self.start_x
    }

    pub fn get_sy(&self) -> i32 {
        self.start_y
    }

    pub fn get_ex(&self) -> i32 {
        self.end_x
    }

    pub fn get_ey(&self) -> i32 {
        self.end_y
    }

    pub fn is_in_zone(&self, x: i32, y: i32) -> bool {
        x >= self.start_x && x < self.end_x && y >= self.start_y && y < self.end_y
    }
}

impl Renderable for Zone {
    fn render(&self, grid: &crate::game::grid::Grid, context: &mut web_sys::CanvasRenderingContext2d, sprites: &web_sys::HtmlImageElement, _: &Zone) {
        for y in self.get_sy()..self.get_ey() {
            for x in self.get_sx()..self.get_ex() {
                let tile = grid.get_tile(x, y).expect("Tile not found");
                tile.render(grid, context, sprites, self);
            }
        };
    }
}
