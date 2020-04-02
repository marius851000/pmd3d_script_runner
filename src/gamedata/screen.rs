use crate::gamedata::{Color, Time};

#[derive(Debug)]
struct ColorTransition {
    elapsed_time: Time,
    duration: Time,
    initial_color: Color,
    final_color: Color,
}

#[derive(Debug)]
pub struct Screen {
    need_update: bool,
    pub actual_color: Color,
    ongoing_transition: Option<ColorTransition>,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            need_update: false,
            actual_color: Color::transparent(),
            ongoing_transition: None,
        }
    }

    pub fn set_color_immediate(&mut self, dest_color: Color) {
        self.actual_color = dest_color
    }

    pub fn set_color_transition(&mut self, duration: Time, dest_color: Color) {
        self.need_update = true;
        self.ongoing_transition = Some(ColorTransition {
            elapsed_time: Time::new(0.0),
            duration,
            initial_color: self.actual_color.clone(),
            final_color: dest_color,
        })
    }

    pub fn time_spent(&mut self, time: &Time) {
        if !self.need_update {
            return;
        };
        let mut still_need_update = false;

        if let Some(transition) = &mut self.ongoing_transition {
            transition.elapsed_time += *time;
            let elapsed_second = transition.elapsed_time.get_time();
            let duration_second = transition.duration.get_time();
            if elapsed_second >= duration_second {
                self.actual_color = transition.final_color.clone();
                self.ongoing_transition = None;
            } else {
                let proportion = (elapsed_second / duration_second) as f32;
                self.actual_color = Color {
                    r: transition.initial_color.r
                        + (transition.final_color.r - transition.initial_color.r) * proportion,
                    g: transition.initial_color.g
                        + (transition.final_color.g - transition.initial_color.g) * proportion,
                    b: transition.initial_color.b
                        + (transition.final_color.b - transition.initial_color.b) * proportion,
                    a: transition.initial_color.a
                        + (transition.final_color.a - transition.initial_color.a) * proportion,
                };
                still_need_update = true;
            };
        };

        self.need_update = still_need_update;
    }
}
