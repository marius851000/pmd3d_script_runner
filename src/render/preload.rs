use std::thread;
use std::rc::Rc;

// Dynamic loading related stuff

pub enum PreLoadState {
    NotLoading,
    Loading,
    Failed,
    Loaded,
}

pub struct PreLoad<Finished, Intermediary> {
    pub state: PreLoadState,
    handle: Option<thread::JoinHandle<Intermediary>>,
    result: Option<Rc<Finished>>
}

impl<Finished, Intermediary> PreLoad<Finished, Intermediary> {
    pub fn new_empty() -> PreLoad<Finished, Intermediary> {
        PreLoad {
            state: PreLoadState::NotLoading,
            handle: None,
            result: None,
        }
    }

    pub fn set_status_loading(&mut self, handle: thread::JoinHandle<Intermediary>) {
        match self.state {
            PreLoadState::NotLoading => self.handle = Some(handle),
            PreLoadState::Loading => {panic!("want to preload something which is already loading!!! ignoring it.")},
            PreLoadState::Failed => {self.handle = Some(handle)},
            PreLoadState::Loaded => {panic!("want to preload something which is already fully loaded!!! ignoring it.")},
        };
        self.state = PreLoadState::Loading;
    }

    pub fn join(&mut self) -> Intermediary {
        match self.state {
            PreLoadState::NotLoading => panic!("no content is actually loading"),
            PreLoadState::Loading => match self.handle.take() {
                Some(handle) => handle.join().unwrap(),
                None => panic!("the value is already taken"),
            },
            PreLoadState::Failed => panic!("the content previously returned a failure"),
            PreLoadState::Loaded => panic!("the content is already loaded"),
        }
    }

    pub fn set_result(&mut self, result: Finished) {
        match self.state {
            PreLoadState::NotLoading => (),
            PreLoadState::Loading => self.handle = None,
            PreLoadState::Failed => (),
            PreLoadState::Loaded => panic!("trying to set a content while it was already set!!! overwriting it"),
        };
        self.state = PreLoadState::Loaded;
        self.result = Some(Rc::new(result));
    }

    pub fn get_result(&mut self) -> Rc<Finished> {
        match self.result.take() { //TODO: quite hacky
            None => panic!("Impossible to get a result, as it not yet computed !"),
            Some(value) => {
                let result = value.clone();
                self.result = Some(value);
                result
            },
        }
    }
}
