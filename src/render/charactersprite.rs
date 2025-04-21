use crate::gamedata::Time;
use crate::render::{WanHandler, WanSprite};
use piston_window::*;
use std::rc::Rc;

pub struct CharacterSprite {
    handler: WanHandler,
    change_with_angle: bool,
    anim_id: usize,
    time_before_next_frame: Time,
    time_between_frame: Time,
}

impl CharacterSprite {
    pub fn new_from_wan_sprite(sprite: Rc<WanSprite>) -> CharacterSprite {
        Self::new_from_wan_handler(WanHandler::new(sprite, true))
    }

    pub fn new_from_wan_handler(handler: WanHandler) -> CharacterSprite {
        let mut result = CharacterSprite {
            handler,
            change_with_angle: false,
            anim_id: 0,
            time_before_next_frame: Time::new(0.1 / 6.0),
            time_between_frame: Time::new(0.1 / 6.0),
        };
        result.set_animation(0, false);
        result
    }

    #[allow(dead_code)]
    pub fn set_shadow(&mut self, shadow_enabled: bool) {
        self.handler.with_shadow = shadow_enabled;
    }

    pub fn set_animation(&mut self, anim_id: usize, change_with_angle: bool) {
        self.change_with_angle = change_with_angle;
        self.anim_id = anim_id;
        self.handler.start_animation(anim_id, 0);
    }

    pub fn draw(
        &mut self,
        graphic: &mut G2d,
        context: &Context,
        coord: &(f64, f64),
        scale: f64,
        angle: f64,
    ) {
        if self.change_with_angle {
            let anim_angle_change =
                (((angle / std::f64::consts::FRAC_PI_4).round() + 4.0) as usize + 6) % 8;
            let anim_angle_change = if anim_angle_change == 8 {
                0
            } else {
                anim_angle_change
            };

            self.handler
                .transmute_animation(self.anim_id, anim_angle_change);
        };
        self.handler.draw_frame(graphic, context, coord, scale);
    }

    pub fn time_spent(&mut self, time: Time) {
        self.time_before_next_frame -= time;
        while self.time_before_next_frame <= Time(0.0) {
            self.handler.next_frame();
            self.time_before_next_frame += self.time_between_frame;
        }
    }
}
