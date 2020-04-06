use crate::gamedata::{Chara, Lock, Screen, Update};
use std::collections::HashMap;
use std::mem::swap;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Debug)]
pub struct Scene {
    pub charas: HashMap<String, Chara>,
    updates: Vec<Update>,
    locks: Vec<Lock>,
    pub screens: Vec<Screen>, //Screen 0: upper, Screen 1: down
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            charas: HashMap::new(),
            updates: Vec::new(),
            locks: Vec::new(),
            screens: vec![Screen::new(), Screen::new()],
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
                    #[allow(unreachable_patterns)]
                    _ => false,
                });
                for screen in &mut self.screens {
                    screen.time_spent(*time);
                }
                for chara in self.charas.values_mut() {
                    chara.time_spent(*time);
                }
            }
            Update::SetScreenColor(screen_id, color) => {
                self.screens[*screen_id as usize].set_color_immediate(color.clone())
            }
            Update::TransitionScreenColor(screen_id, duration, color) => self.screens
                [*screen_id as usize]
                .set_color_transition(duration.clone(), color.clone()),
        };
        self.updates.push(update);
    }

    pub fn check_lock(&mut self) {
        let mut finished = Vec::new();
        for (id, lock) in self.locks.iter().enumerate() {
            if lock.is_finished(&self) {
                finished.push(id)
            }
        }

        for (difference, original_id) in finished.iter().enumerate() {
            let true_id = original_id - difference;
            self.locks[true_id].unlock();
        }
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
