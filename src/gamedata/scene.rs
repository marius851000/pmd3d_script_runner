use drain_filter_polyfill::VecExt;

use crate::gamedata::{Chara, Lock, Portrait, Screen, Update};
use std::collections::HashMap;
use std::mem::swap;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Debug)]
pub struct Scene {
    pub charas: HashMap<String, Chara>,
    updates: Vec<Update>,
    locks: Vec<Lock>,
    pub screens: Vec<Screen>, //Screen 0: upper, Screen 1: down
    pub portrait: Option<Portrait>,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            charas: HashMap::new(),
            updates: Vec::new(),
            locks: Vec::new(),
            screens: vec![Screen::new(), Screen::new()],
            portrait: None,
        }
    }
}

impl Scene {
    pub fn update(&mut self, update: Update) {
        match &update {
            Update::AddChara(id, actor) => {
                //TODO: There are ligmited named slots for characters in PSMD, so only existing (as determined by a plb?) character names can be used.
                //Nonexistent names all refer to the same character.
                //For example, the following will create a new Axew character, accessible by CH("Orus").
                //CHARA:DynamicLoad("Orus", "KIBAGO") --(name, actor)
                //However, any nonexistent name with refer to this character, even CH("asdfg")
                //As a result, the following will conflict with the already existing character and produce an error.
                //CHARA:DynamicLoad("Laurenna", "TSUTAAJA")
                //Surprisingly, this works as intended in Gates (go figure).
                let chara = Chara::new(actor.clone());
                self.charas.insert(id.clone(), chara);
            }
            Update::DelChara(id) => {
                self.charas.remove(id);
            }
            Update::SetPosition(id, position) => {
                self.charas
                    .get_mut(id)
                    .unwrap()
                    .set_position(position.clone());
            }
            Update::WalkTo(id, position, speed) => {
                self.charas
                    .get_mut(id)
                    .unwrap()
                    .walk_to(position.clone(), speed.clone());
            }
            Update::AddLock(lock) => self.locks.push(lock.clone()),
            Update::TimeSpent(time) => {
                self.locks.drain_filter(|lock| match lock {
                    Lock::Wait(lock, remaining_time) => {
                        *remaining_time -= *time;
                        if remaining_time.get_time() < 0.0 {
                            lock.store(true, Relaxed);
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                });
                for screen in &mut self.screens {
                    screen.time_spent(*time);
                }
                for (charid, chara) in self.charas.iter_mut() {
                    if chara.time_spent(*time) {
                        self.updates.push(Update::StartIDLE(charid.clone()));
                        self.locks.drain_filter(|lock| match lock {
                            Lock::WaitMove(lock, lock_charid) => {
                                if charid == lock_charid {
                                    lock.store(true, Relaxed);
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => false,
                        });
                        ()
                    };
                }
            }
            Update::SetScreenColor(screen_id, color) => {
                self.screens[*screen_id as usize].set_color_immediate(color.clone())
            }
            Update::TransitionScreenColor(screen_id, duration, color) => self.screens
                [*screen_id as usize]
                .set_color_transition(duration.clone(), color.clone()),
            Update::StartIDLE(_) => (),
            Update::SetPortrait(portrait) => self.portrait = Some(portrait.clone()),
            Update::RemovePortrait => self.portrait = None,
        };
        if log_enabled!(log::Level::Debug) {
            match update {
                Update::TimeSpent(_) => (),
                _ => {
                    debug!("new update: {:?}", update);
                    trace!("list of locks: {:?}", self.locks);
                }
            };
        };
        self.updates.push(update);
    }

    pub fn get_and_clear_updates(&mut self) -> Vec<Update> {
        let mut replace = Vec::new();
        swap(&mut self.updates, &mut replace);
        replace
    }
}

#[test]
fn test_scene_get_updates() {
    let mut scene = Scene::default();
    let update = Update::AddChara("HERO".into(), "KIBAGO".into());
    scene.update(update.clone());
    assert!(!scene.get_and_clear_updates().is_empty());
    assert!(scene.get_and_clear_updates().is_empty());
}
