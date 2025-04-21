extern crate piston_window;
use crate::render::{PreLoad, PreLoadState};
use ::image::{ImageBuffer, Rgba};
use piston_window::*;
use pmd_cpack::CPack;
use pmd_pkdpx::{decompress_px, is_px};
use pmd_wan::{
    AnimationStore, FragmentFlip, FragmentResolution, FrameStore,
    WanImage as WanImg,
};
use std::collections::HashMap;
use std::io::Cursor;
use std::io::{Read, Seek};
use std::thread;
use std::{rc::Rc, sync::Arc};

pub struct WanStore<F: 'static + Read + Seek + Send> {
    pack: Arc<CPack<F>>,
    sprites: Vec<PreLoad<WanSprite, WanImg>>,
}

impl<F: 'static + Read + Seek + Send> WanStore<F> {
    pub fn new(pack: CPack<F>) -> WanStore<F> {
        let mut sprites = Vec::new();
        for _ in 0..pack.len() {
            sprites.push(PreLoad::new_empty());
        }
        WanStore {
            pack: Arc::new(pack),
            sprites: sprites,
        }
    }

    pub fn preload_sprite(&mut self, sprite_id: usize) -> () {
        if sprite_id >= self.pack.len() {
            panic!("the sprite id does not exist !");
        }
        match &self.sprites[sprite_id].state {
            PreLoadState::Loading => return (), //TODO: rather check if it finished, and if there was an error, retry it.
            PreLoadState::Loaded => return (),
            _ => (),
        }
        let pack = self.pack.clone();
        let handle = thread::spawn(move || {
            let mut file = pack.get_file(sprite_id).unwrap();
            if is_px(&mut file).unwrap() {
                WanImg::decode_wan(Cursor::new(decompress_px(file).unwrap())).unwrap()
            } else {
                WanImg::decode_wan(file).unwrap()
            }
        });
        self.sprites[sprite_id].set_status_loading(handle);
        ()
    }

    pub fn get_sprite(
        &mut self,
        texture_context: &mut G2dTextureContext,
        sprite_id: usize,
    ) -> Rc<WanSprite> {
        if sprite_id >= self.pack.len() {
            panic!("the sprite id does not exist !");
        };
        match &self.sprites[sprite_id].state {
            PreLoadState::NotLoading => self.preload_sprite(sprite_id),
            _ => (),
        };

        match &self.sprites[sprite_id].state {
            PreLoadState::Loading => {
                let content = self.sprites[sprite_id].join();
                self.sprites[sprite_id]
                    .set_result(WanSprite::new_from_wan(content, texture_context));
            }
            _ => (),
        };

        self.sprites[sprite_id].get_result()
    }
}

pub struct WanHandler {
    sprite: Rc<WanSprite>,
    frame_time_position: usize,
    animation_variation: usize,
    animation_id: usize,
    looped_at_least_1: bool,
    animation_loaded: bool,
    pub with_shadow: bool,
}

impl WanHandler {
    pub fn new(sprite: Rc<WanSprite>, with_shadow: bool) -> WanHandler {
        WanHandler {
            sprite: sprite,
            frame_time_position: 0,
            animation_variation: 0,
            animation_id: 0,
            looped_at_least_1: false,
            animation_loaded: false,
            with_shadow: with_shadow,
        }
    }

    pub fn start_animation(&mut self, animation_id: usize, variation: usize) -> () {
        if animation_id >= self.sprite.len_animations() {
            panic!("WanHandler::start_anim: impossible to set an animation, as it doesn't exist.");
        };
        self.animation_id = animation_id;
        self.animation_variation = variation;
        self.animation_loaded = true;
        self.looped_at_least_1 = false;
        self.frame_time_position = 0;
    }

    pub fn transmute_animation(&mut self, animation_id: usize, animation_variation: usize) -> () {
        if animation_id >= self.sprite.len_animations() {
            panic!("WanHandler::start_anim: impossible to set an animation, as it doesn't exist.");
        };
        let anim_duration = self.sprite.duration_anim(animation_id, animation_variation);
        self.animation_id = animation_id;
        self.animation_variation = animation_variation;
        if self.frame_time_position >= anim_duration {
            self.frame_time_position = 0;
        };
    }

    pub fn draw_frame(
        &self,
        graphic: &mut G2d,
        context: &Context,
        coord: &(f64, f64),
        scale: f64,
    ) -> () {
        if !self.animation_loaded {
            panic!("no animation is loaded for a WanHandler !!!");
            //TODO: play the first one instead
        };
        self.sprite.draw_animation(
            graphic,
            &context,
            self.animation_id,
            self.animation_variation,
            self.frame_time_position as u16,
            self.with_shadow,
            coord,
            scale,
        );
    }

    pub fn next_frame(&mut self) {
        if !self.animation_loaded {
            panic!();
        };
        self.frame_time_position += 1;
        if self.frame_time_position
            >= self
                .sprite
                .duration_anim(self.animation_id, self.animation_variation)
        {
            self.looped_at_least_1 = true;
            self.frame_time_position = 0;
        }
    }

    #[allow(dead_code)]
    pub fn is_finished(&self) -> bool {
        if self.looped_at_least_1 {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn len_animations(&self) -> usize {
        self.sprite.len_animations()
    }
}

pub struct WanHostFragment {
    texture: G2dTexture,
    width: f32,
    height: f32,
}

impl WanHostFragment {
    fn new(
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        texture_context: &mut G2dTextureContext,
    ) -> WanHostFragment {
        let texture =
            Texture::from_image(texture_context, image, &TextureSettings::new()).unwrap();

        WanHostFragment {
            texture: texture,
            width: image.width() as f32,
            height: image.height() as f32,
        }
    }

    fn draw(
        &self,
        graphic: &mut G2d,
        context: &Context,
        coord: &(f64, f64),
        scale: f64,
        flip: FragmentFlip,
    ) -> () {
        /*if flip.0 || flip.1 {
            canvas.get_renderer().copy_ex(
                &self.texture,
                None,
                Some(Rect::new(coord.0,coord.1,(self.width*scale) as u32,(self.height*scale) as u32)),
                0.0,
                None,
                flip.0,
                flip.1,
            ).unwrap();
        } else {
            canvas.get_renderer().copy(
                &self.texture,
                None,
                Some(Rect::new(coord.0,coord.1,(self.width*scale) as u32,(self.height*scale) as u32)),
            ).unwrap();
        }*/
        //TODO: flip
        let scaled_width = scale * self.width as f64;
        let scaled_height = scale * self.height as f64;
        /*let scaled_width = 0.0;
        let scaled_height = 0.0;*/
        image(
            &self.texture,
            context
                .transform
                .trans(
                    coord.0 + if flip.flip_h { scaled_width } else { 0.0 },
                    coord.1 + if flip.flip_v { scaled_height } else { 0.0 },
                )
                .scale(
                    if flip.flip_h { -scale } else { scale },
                    if flip.flip_v { -scale } else { scale },
                ),
            graphic,
        );
    }
}

pub struct WanSprite {
    fragment_host: HashMap<(usize, (u8, u8)), WanHostFragment>,
    frames: FrameStore,
    animations: AnimationStore,
}

impl WanSprite {
    pub fn new_from_wan(wan: WanImg, texture_context: &mut G2dTextureContext) -> WanSprite {
        let mut fragment_host = HashMap::default();
        for frame in &wan.frame_store.frames {
            for fragment in &frame.fragments {
                let key = (
                    fragment.fragment_bytes_index,
                    (fragment.resolution.x, fragment.resolution.y),
                );
                if !fragment_host.contains_key(&key) {
                    fragment_host.insert(
                        key,
                        WanHostFragment::new(
                            &wan.fragment_bytes_store.fragment_bytes[fragment.fragment_bytes_index]
                                .get_image(&wan.palette, &fragment.resolution, fragment.pal_idx)
                                .unwrap(),
                            texture_context,
                        ),
                    );
                }
            }
        }

        WanSprite {
            fragment_host,
            frames: wan.frame_store,
            animations: wan.animation_store,
        }
    }

    fn draw_fragment(
        &self,
        graphic: &mut G2d,
        context: &Context,
        fragment_bytes_id: usize,
        resolution: FragmentResolution,
        coord: &(f64, f64),
        scale: f64,
        flip: FragmentFlip,
    ) -> () {
        self.fragment_host
            .get(&(fragment_bytes_id, (resolution.x, resolution.y)))
            .unwrap()
            .draw(graphic, context, coord, scale, flip);
    }

    fn draw_frame(
        &self,
        graphic: &mut G2d,
        context: &Context,
        frame_id: usize,
        coord: &(f64, f64),
        scale: f64,
    ) -> () {
        for fragment in &self.frames.frames[frame_id].fragments {
            let offset_x = ((fragment.offset_x as f64) * scale) as f64;
            let offset_y = ((fragment.offset_y as f64) * scale) as f64;
            self.draw_fragment(
                graphic,
                context,
                fragment.fragment_bytes_index,
                fragment.resolution,
                &(coord.0 + offset_x, coord.1 + offset_y),
                scale,
                fragment.flip,
            );
        }
    }

    pub fn draw_animation(
        &self,
        graphic: &mut G2d,
        context: &Context,
        animation_id: usize,
        animation_variation: usize,
        frame_time_position: u16,
        with_shadow: bool,
        coord: &(f64, f64),
        scale: f64,
    ) -> () {
        //TODO: support defining the orientation
        if animation_id >= self.animations.anim_groups.len() {
            panic!("the animation id is superior to the number of animation");
        };
        let mut animation_frame = None;
        let mut loop_frame_time_position = 0;
        for current_animation_frame in
            &self.animations.anim_groups[animation_id][animation_variation].frames
        {
            loop_frame_time_position += current_animation_frame.duration as u16;
            if loop_frame_time_position >= frame_time_position {
                animation_frame = Some(current_animation_frame);
                break;
            };
        }

        match animation_frame {
            None => panic!("the frame number is too high, and is not found in the animation."),
            Some(animation_frame) => {
                let coord_x = coord.0 + (((animation_frame.offset_x as f64) * scale) as f64);
                let coord_y = coord.1 + (((animation_frame.offset_y as f64) * scale) as f64);
                //draw shadow
                if with_shadow {
                    let shadow_x =
                        coord.0 + ((animation_frame.shadow_offset_x as f64) * scale) as f64;
                    let shadow_y =
                        coord.1 + ((animation_frame.shadow_offset_y as f64) * scale) as f64;
                    let shadow_coeff = (10.0 * scale) as f64;
                    rectangle(
                        [0.0, 0.0, 0.0, 1.0],
                        [
                            shadow_x - (shadow_coeff),
                            shadow_y - (shadow_coeff),
                            shadow_coeff * 2.0,
                            shadow_coeff * 2.0,
                        ],
                        context.transform,
                        graphic,
                    );
                };
                self.draw_frame(
                    graphic,
                    context,
                    animation_frame.frame_id as usize,
                    &(coord_x, coord_y),
                    scale,
                );
            }
        };
    }

    pub fn len_animations(&self) -> usize {
        self.animations.anim_groups.len()
    }

    pub fn duration_anim(&self, animation_id: usize, animation_variation: usize) -> usize {
        if animation_id >= self.len_animations() {
            panic!("impossible to get the lenght of animation, as the given animation id does not exist.");
        };
        if animation_variation >= self.animations.anim_groups[animation_id].len() {
            panic!("impossible to get the lenght of animation, as the given animation variation does not exist.");
        }
        return self.animations.anim_groups[animation_id][animation_variation]
            .frames
            .iter()
            .map(|x| x.duration as usize)
            .sum();
    }
}
