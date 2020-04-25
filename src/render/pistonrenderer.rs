extern crate piston_window;
use crate::gamedata::Update;
use crate::render::Camera;
use crate::render::CharacterSprite;
use crate::render::WanStore;
use crate::Input;
use crate::Logic;
use piston_window::*;
use pmd_cpack::CPack;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

pub struct PistonRenderer {
    window: PistonWindow,
    logic: Option<Logic>,
    image_store: Option<WanStore<File>>,
    characters_sprite: Option<HashMap<String, CharacterSprite>>,
}

impl PistonRenderer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PistonRenderer {
            window: WindowSettings::new("Hello Piston!", [640, 480])
                .exit_on_esc(true)
                .build()
                .unwrap(),
            logic: None,
            image_store: None,
            characters_sprite: Some(HashMap::new()),
        }
    }

    pub fn load(&mut self, code: &str) {
        self.logic = Some(Logic::new(code));
        //TODO: do not hardcode the path
        self.image_store = Some(WanStore::new(
            CPack::new_from_file(File::open(PathBuf::from("data/MONSTER/m_ground.bin")).unwrap())
                .unwrap(),
        ));
    }

    pub fn run(&mut self) {
        let image_store = self.image_store.as_mut().unwrap();
        let logic: &mut Logic = self.logic.as_mut().unwrap();
        let characters_sprite = self.characters_sprite.as_mut().unwrap();

        // What does a position unit represent in pixel ? (float)
        let scale = 100.0;
        let scale_div2 = scale / 2.0;
        let mut camera = Camera::new(scale, (0.0, 0.0), 0.0);
        while let Some(e) = self.window.next() {
            if let Event::Loop(Loop::Update(update_arg)) = e {
                logic.execute(Input::new(update_arg.dt));
                for update in logic.get_and_clear_updates() {
                    match update {
                        Update::AddChara(charid, actor) => {
                            let spriteid = match actor.as_str() {
                                "KIBAGO" => 3,
                                "TSUTAAJA" => 6,
                                _ => 597,
                            };
                            let wan_sprite = image_store
                                .get_sprite(&mut self.window.create_texture_context(), spriteid);
                            let mut spr = CharacterSprite::new_from_wan_sprite(wan_sprite);
                            spr.set_animation(16, true);
                            characters_sprite.insert(charid, spr);
                        }
                        Update::DelChara(charid) => {
                            characters_sprite.remove(&charid);
                        }
                        Update::TimeSpent(time) => {
                            for chara in characters_sprite.values_mut() {
                                chara.time_spent(time);
                            }
                        }
                        Update::WalkTo(charid, _, _) => {
                            characters_sprite
                                .get_mut(&charid)
                                .unwrap()
                                .set_animation(0, true);
                        }
                        Update::StartIDLE(charid) => {
                            characters_sprite
                                .get_mut(&charid)
                                .unwrap()
                                .set_animation(16, true);
                        }
                        _ => (),
                    }
                }
            };

            let scene_arc = logic.scene.clone();

            self.window.draw_2d(&e, |c, g, _device| {
                let scene = scene_arc.lock().unwrap();
                let viewport = c.viewport.as_ref().unwrap();
                let screen_x = viewport.draw_size[0] as f64;
                let screen_y = viewport.draw_size[1] as f64;
                camera.set_screen_size((screen_x, screen_y));
                //clear the screen
                clear([1.0; 4], g);
                //draw characters
                for (charaid, chara) in scene.charas.iter() {
                    let display_data =
                        camera.compute_display_data((chara.position.x, -chara.position.y), 0.0);
                    characters_sprite.get_mut(charaid).unwrap().draw(
                        g,
                        &c,
                        &(display_data.x_pixel, display_data.y_pixel),
                        scale / 32.0,
                        chara.angle,
                    );
                }
                //render the front screen
                rectangle(
                    [
                        scene.screens[0].actual_color.r,
                        scene.screens[0].actual_color.g,
                        scene.screens[0].actual_color.b,
                        scene.screens[0].actual_color.a,
                    ],
                    [0.0, 0.0, screen_x, screen_y],
                    c.transform,
                    g,
                );
            });
        }
    }

    pub fn close(&mut self) {}
}
