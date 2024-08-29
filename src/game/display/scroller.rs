use super::zone::Zone;

#[derive(Debug)]
pub struct Scroller {
    from_zone: Zone,
    to_zone: Zone,
    active_zone: Option<Zone>,
    x_counter: i32,
    y_counter: i32,
    x_reached: bool,
    y_reached: bool,
}

impl Scroller {
    pub fn new(from_zone: Zone, to_zone: Zone) -> Self {
        Self {
            from_zone,
            to_zone,
            active_zone: None,
            x_counter: 0,
            y_counter: 0,
            x_reached: false,
            y_reached: false,
        }
    }

    pub fn update(&mut self) -> Option<Zone> {
        if let Some(active_zone) = self.active_zone {
            if active_zone == self.to_zone {
                self.active_zone = None;
                return None;
            } else if active_zone.get_sx() == self.to_zone.get_sx() && active_zone.get_ex() == self.to_zone.get_ex() {
                self.x_reached = true;
            } else if active_zone.get_sy() == self.to_zone.get_sy() && active_zone.get_ey() == self.to_zone.get_ey() {
                self.y_reached = true;
            }
        } else if self.from_zone == self.to_zone {
            return None;
        }

        if !self.x_reached {
            self.x_counter += 1;
        } 
        if !self.y_reached {
            self.y_counter += 1;
        }

        let mut new_sx = self.from_zone.get_sx();
        let mut new_ex = self.from_zone.get_ex();
        let mut new_sy = self.from_zone.get_sy();
        let mut new_ey = self.from_zone.get_ey();

        if self.from_zone.get_ex() > self.to_zone.get_ex() {
            new_sx -= self.x_counter;
            new_ex -= self.x_counter;
        } else if self.from_zone.get_ex() < self.to_zone.get_ex() {
            new_sx += self.x_counter;
            new_ex += self.x_counter;
        }

        if self.from_zone.get_ey() > self.to_zone.get_ey() {
            new_sy -= self.y_counter;
            new_ey -= self.y_counter;
        } else if self.from_zone.get_ey() < self.to_zone.get_ey() {
            new_sy += self.y_counter;
            new_ey += self.y_counter;
        }

        self.active_zone = Some(Zone::new(new_sx, new_ex, new_sy, new_ey));
        self.active_zone
    }

    pub fn get_active_zone(&self) -> Option<Zone> {
        self.active_zone
    }
}

