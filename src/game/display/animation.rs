use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::game::{enums::{action_type::ActionType, animation_type::AnimationType}, grid::Grid, interfaces::renderable::Renderable};

use super::{action::Action, zone::Zone};

#[derive(Clone, Copy, Debug)]
pub struct Animation {
    animation_type: AnimationType,
    duration: i32,
    frame: u32,
    position: (i32, i32),
}

impl Animation {
    pub fn new(animation_type: AnimationType, duration: i32, position: (i32, i32)) -> Self {
        Self {
            animation_type,
            duration,
            frame: 0,
            position,
        }
    }

    pub fn from_action(action: &Action) -> Option<Self> {
        match action.get_action_type() {
            ActionType::KillPlayer => Some(Self::new(AnimationType::Death, 6, action.get_position())),
            _ => None,
        }
    }

    pub fn update(&mut self) -> Option<i32> {
        if self.frame < self.duration as u32 {
            self.frame += 1;
            Some(self.duration)
        } else {
            None
        }
    }

    fn render_death(&self, _: &mut CanvasRenderingContext2d, _: &HtmlImageElement, _: &Zone) {
        
    }

    fn render_spawn(&self, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        let sy = 6.0;
        let sx: f64;
        let (dx, dy) = zone.get_patched_position(self.position);
        if self.frame % 2 == 0 {
            sx = 1.0;
        } else {
            sx = 2.0;
        };
        let _ = context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            sprites,
            sx * 32.0,
            sy * 32.0,
            32.0,
            32.0,
            dx,
            dy + 32.0,
            32.0,
            32.0,
        );
    }
}

impl Renderable for Animation {
    fn render(&self, _: &Grid, context: &mut CanvasRenderingContext2d, sprites: &HtmlImageElement, zone: &Zone) {
        match self.animation_type {
            AnimationType::Death => self.render_death(context, sprites, zone),
            AnimationType::Spawn => self.render_spawn(context, sprites, zone),
        };
    }
}
