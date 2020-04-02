extern crate piston_window;
use crate::Logic;
use crate::Input;
use crate::render::Camera;
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
        // What does a position unit represent in pixel ? (float)
        let scale = 100.0;
        let scale_div2 = scale/2.0;
        let mut camera = Camera::new(scale, (0.0, 0.0), 0.0);
        while let Some(e) = self.window.next() {
            match e {
                Event::Loop(Loop::Update(update_arg)) => logic.execute(Input::new(update_arg.dt)),
                _ => ()
            };

            let scene_arc = logic.scene.clone();
            self.window.draw_2d(&e, |c, g, _device| {
                let scene = scene_arc.lock().unwrap();
                let viewport = c.viewport.as_ref().unwrap();
                let screen_x = viewport.draw_size[0] as f64;
                let screen_y = viewport.draw_size[1] as f64;
                println!("screen_x: {}, screen_y: {}", screen_x, screen_y);
                camera.set_screen_size((screen_x, screen_y));
                //clear the screen
                clear([1.0; 4], g);
                //draw charas
                for (_chara_id, chara) in &scene.charas {
                    let display_data = camera.compute_display_data((chara.position.x, chara.position.y), 0.0);
                    println!("{:?}", chara);
                    println!("{:?}", display_data);
                    //TODO: draw the angle
                    rectangle(
                        [0.5, 0.5, 0.5, 1.0],
                        [display_data.x_pixel-scale_div2, display_data.y_pixel-scale_div2, scale, scale],
                        c.transform,
                        g,
                    )
                };
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
