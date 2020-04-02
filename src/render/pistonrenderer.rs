extern crate piston_window;
use crate::Logic;
use crate::Input;
use piston_window::*;

pub struct PistonRenderer {
    window: PistonWindow,
    logic: Option<Logic>,
}

impl PistonRenderer {
    pub fn new() -> Self {
        PistonRenderer {
            window: WindowSettings::new("Hello Piston!", [640, 480])
                .exit_on_esc(true)
                .build()
                .unwrap(),
            logic: None,
        }
    }

    pub fn load(&mut self, code: &str) {
        self.logic = Some(Logic::new(code));
    }

    pub fn run(&mut self) {
        let logic: &mut Logic = self.logic.as_mut().unwrap();
        while let Some(e) = self.window.next() {
            match e {
                Event::Loop(Loop::Update(update_arg)) => logic.execute(Input::new(update_arg.dt)),
                _ => ()
            };

            let scene_arc = logic.scene.clone();
            self.window.draw_2d(&e, move |c, g, _device| {
                let scene = scene_arc.lock().unwrap();
                let viewport = c.viewport.as_ref().unwrap();
                let screen_x = viewport.draw_size[0] as f64;
                let screen_y = viewport.draw_size[1] as f64;
                //clear the screen
                clear([1.0; 4], g);
                //render the front screen
                rectangle(
                    [scene.screens[0].actual_color.r, scene.screens[0].actual_color.g, scene.screens[0].actual_color.b, scene.screens[0].actual_color.a],
                    [0.0, 0.0, screen_x, screen_y],
                    c.transform,
                    g,
                );
            });
        }
    }

    pub fn close(&mut self) {
    }
}
