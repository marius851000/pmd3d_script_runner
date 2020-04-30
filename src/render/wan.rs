extern crate piston_window;
use crate::render::{PreLoad, PreLoadState};
use ::image::ImageBuffer;
use piston_window::*;
use pmd_cpack::CPack;
use pmd_pkdpx::{decompress_px, is_px};
use pmd_wan::wan::{AnimStore, MetaFrameStore};
use pmd_wan::WanImage as WanImg;
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
                WanImg::new(Cursor::new(decompress_px(file).unwrap())).unwrap()
            } else {
                WanImg::new(file).unwrap()
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
    frame: usize,
    animation: usize,
    looped_at_least_1: bool,
    animation_len: usize,
    animation_loaded: bool,
    pub with_shadow: bool,
}

impl WanHandler {
    pub fn new(sprite: Rc<WanSprite>, with_shadow: bool) -> WanHandler {
        WanHandler {
            sprite: sprite,
            frame: 0,
            animation: 0,
            looped_at_least_1: false,
            animation_len: 1,
            animation_loaded: false,
            with_shadow: with_shadow,
        }
    }

    pub fn start_animation(&mut self, animation_id: usize) -> () {
        if animation_id >= self.sprite.len_animations() {
            panic!("WanHandler::start_anim: impossible to set an animation, as it doesn't exist.");
        };
        self.animation_len = self.sprite.len_anim(animation_id);
        self.animation = animation_id;
        self.animation_loaded = true;
        self.looped_at_least_1 = false;
        self.frame = 0;
    }

    pub fn transmute_animation(&mut self, animation_id: usize) -> () {
        if animation_id >= self.sprite.len_animations() {
            panic!("WanHandler::start_anim: impossible to set an animation, as it doesn't exist.");
        };
        let animation_len = self.sprite.len_anim(animation_id);
        self.animation = animation_id;
        if animation_len != self.animation_len {
            self.animation_len = animation_len;
            if self.frame >= animation_len {
                self.frame = 0;
            };
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
        if self.frame >= std::u16::MAX as usize {
            panic!("the number of frame of an animation has gone to it's maximum!!! That should never happen, as it should return to 0 when the animation end (or crash for any other reason while drawing previous frame) !!! Restarting it anyway")
        }
        self.sprite.draw_animation(
            graphic,
            &context,
            self.animation,
            self.frame as u16,
            self.with_shadow,
            coord,
            scale,
        );
    }

    pub fn next_frame(&mut self) {
        if !self.animation_loaded {
            panic!();
        };
        self.frame += 1;
        if self.frame >= self.animation_len {
            self.looped_at_least_1 = true;
            self.frame = 0;
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

pub struct WanImage {
    texture: G2dTexture,
    width: f32,
    height: f32,
}

impl WanImage {
    fn new(
        data: Vec<u8>,
        width: u32,
        height: u32,
        texture_context: &mut G2dTextureContext,
    ) -> WanImage {
        let surface = ImageBuffer::from_raw(width, height, data).unwrap(); //TODO: see if there isn't an already existing image -> Surface

        let texture =
            Texture::from_image(texture_context, &surface, &TextureSettings::new()).unwrap();

        WanImage {
            texture: texture,
            width: width as f32,
            height: height as f32,
        }
    }

    fn draw(
        &self,
        graphic: &mut G2d,
        context: &Context,
        coord: &(f64, f64),
        scale: f64,
        flip: (bool, bool),
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
                    coord.0 + if flip.0 { scaled_width } else { 0.0 },
                    coord.1 + if flip.1 { scaled_height } else { 0.0 },
                )
                .scale(
                    if flip.0 { -scale } else { scale },
                    if flip.1 { -scale } else { scale },
                ),
            graphic,
        );
    }
}

pub struct WanSprite {
    images: Vec<WanImage>,
    meta_frames: MetaFrameStore,
    animations: AnimStore,
}

impl WanSprite {
    pub fn new_from_wan(wan: WanImg, texture_context: &mut G2dTextureContext) -> WanSprite {
        let mut images = Vec::new();

        for img in &wan.image_store.images {
            let data = img.img.clone().into_raw();
            let image = WanImage::new(data, img.img.width(), img.img.height(), texture_context);
            images.push(image);
        }

        WanSprite {
            images: images,
            meta_frames: wan.meta_frame_store,
            animations: wan.anim_store,
        }
    }

    pub fn draw_image(
        &self,
        graphic: &mut G2d,
        context: &Context,
        image_id: usize,
        coord: &(f64, f64),
        scale: f64,
        flip: (bool, bool),
    ) -> () {
        if image_id >= self.images.len() {
            panic!("the image id is superior to the lenght of the image list.");
        };
        self.images[image_id].draw(graphic, context, coord, scale, flip);
    }

    pub fn draw_meta_frame_group(
        &self,
        graphic: &mut G2d,
        context: &Context,
        meta_frame_group_id: usize,
        coord: &(f64, f64),
        scale: f64,
    ) -> () {
        if meta_frame_group_id >= self.meta_frames.meta_frame_groups.len() {
            panic!("the meta-frame id is superior to the lenght of the meta-frames list");
        };
        let meta_frames = &self.meta_frames.meta_frames;
        for meta_frame_id in &self.meta_frames.meta_frame_groups[meta_frame_group_id].meta_frames_id
        {
            let meta_frame = &meta_frames[*meta_frame_id];
            let image_id = meta_frame.image_index;
            let offset_x = ((meta_frame.offset_x as f64) * scale) as f64;
            let offset_y = ((meta_frame.offset_y as f64) * scale) as f64;
            self.draw_image(
                graphic,
                context,
                image_id,
                &(coord.0 + offset_x, coord.1 + offset_y),
                scale,
                (meta_frame.h_flip, meta_frame.v_flip),
            );
        }
    }

    pub fn draw_animation(
        &self,
        graphic: &mut G2d,
        context: &Context,
        animation_id: usize,
        frame_number: u16,
        with_shadow: bool,
        coord: &(f64, f64),
        scale: f64,
    ) -> () {
        if animation_id >= self.animations.len() {
            panic!("the animation id is superior to the number of animation");
        };
        let mut frame_id = None;
        let mut frame_actual_number: u16 = 0;
        for actual_frame_id in 0..self.animations.animations[animation_id].len() {
            frame_actual_number +=
                self.animations.animations[animation_id].frames[actual_frame_id].duration as u16;
            if frame_actual_number > frame_number {
                frame_id = Some(actual_frame_id);
                break;
            };
        }

        match frame_id {
            None => panic!("the frame number is too high, and is not found in the animation."),
            Some(id) => {
                let frame = &self.animations.animations[animation_id].frames[id];
                let coord_x = coord.0 + (((frame.offset_x as f64) * scale) as f64);
                let coord_y = coord.1 + (((frame.offset_y as f64) * scale) as f64);
                //draw shadow
                if with_shadow {
                    let shadow_x = coord.0 + ((frame.shadow_offset_x as f64) * scale) as f64;
                    let shadow_y = coord.1 + ((frame.shadow_offset_y as f64) * scale) as f64;
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
                self.draw_meta_frame_group(
                    graphic,
                    context,
                    frame.frame_id as usize,
                    &(coord_x, coord_y),
                    scale,
                );
            }
        };
    }

    pub fn len_animations(&self) -> usize {
        self.animations.len()
    }

    pub fn len_anim(&self, animation_id: usize) -> usize {
        if animation_id >= self.len_animations() {
            panic!("impossible to get the lenght of animation, as the given animation id does not exist.");
        };
        let mut frame_number = 0;
        for actual_frame_id in 0..self.animations.animations[animation_id].len() {
            frame_number +=
                self.animations.animations[animation_id].frames[actual_frame_id].duration as usize;
        }
        frame_number
    }

    #[allow(dead_code)]
    pub fn len_meta_frame_group(&self) -> usize {
        self.meta_frames.meta_frame_groups.len()
    }
}
