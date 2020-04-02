//! This file contain everything required to make cooperative multiprocessing lua reader implementation
use rlua::{Context, Function, Lua, Nil, Table, Thread, ThreadStatus, UserData};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc, Mutex,
};

#[derive(Clone, Debug)]
/// The reason a ``Thread`` is locked
pub enum LockReason {
    /// There is no reason for this ``Thread`` to be blocked
    None,
    /// The ``Thread`` wait the ``AtomicBool`` to be true
    WaitAtomicBool(Arc<AtomicBool>),
}

impl LockReason {
    pub fn new_abool(abool: Arc<AtomicBool>) -> LockReason {
        LockReason::WaitAtomicBool(abool)
    }
}

impl LockReason {
    /// Return ``true`` if the ``Thread`` should resume execution, false otherwise
    pub fn can_continue(&self) -> bool {
        match self {
            Self::None => true,
            Self::WaitAtomicBool(value) => return value.load(Relaxed),
        }
    }
}

#[derive(Clone, Debug)]
/// A struct that should be returned when a ``Thread`` is not finished, but want to interupt it's instruction until another event happend
pub struct YieldResult {
    lock: LockReason,
}

impl YieldResult {
    /// Create a new ``YieldResult``. lock is the ``LockReason`` that indicate the external event it wait
    pub fn new(lock: LockReason) -> Self {
        Self { lock }
    }
}

impl UserData for YieldResult {}

#[derive(Default, Debug)]
struct LuaRunningData {
    next_task_id: u64,
    task_look_list: HashMap<u64, LockReason>,
}

impl LuaRunningData {
    fn add_running_thread<'lua>(&mut self, ctx: &Context<'lua>, thread: Thread<'lua>) {
        let globals = ctx.globals();

        let created_task_id = self.next_task_id;
        self.next_task_id += 1;

        let running_table = globals.get::<_, Table>("_yammy_running_coroutine").unwrap();
        running_table.set(created_task_id, thread).unwrap();
        globals
            .set("_yammy_running_coroutine", running_table)
            .unwrap();

        self.task_look_list
            .insert(created_task_id, LockReason::None);
    }

    fn set_running_thread_lock(&mut self, id: u64, reason: LockReason) {
        self.task_look_list.insert(id, reason);
    }

    fn delete_running_thread(&mut self, id: u64) {
        self.task_look_list.remove(&id);
    }

    fn list_run_possibility(&self) -> Vec<u64> {
        self.task_look_list
            .iter()
            .filter_map(|(k, v)| if v.can_continue() { Some(*k) } else { None })
            .collect()
    }
}

/// A running lua script. Multiple ``Thread``s can run in a cooperative parallel mode.
///
/// Inside this environment is avalaible the lua function yammy_fork(function), that will fork the process.
/// Function can also wait for a long time in a non blocking way (TODO: explain how to do that)
pub struct RunningLua {
    lua: Lua,
    running_data: Arc<Mutex<LuaRunningData>>,
}

impl std::fmt::Debug for RunningLua {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunningLua")
            .field("running_data", &self.running_data)
            .finish()
    }
}

impl Default for RunningLua {
    fn default() -> Self {
        let mut lua = RunningLua {
            lua: Lua::new(),
            running_data: Arc::new(Mutex::new(LuaRunningData::default())),
        };
        lua.env_setup();
        lua
    }
}

impl RunningLua {
    pub fn new_from_script(script: &str) -> RunningLua {
        let mut new = Self::default();
        new.load_script(script);
        new
    }

    pub fn load_script(&mut self, script: &str) {
        self.lua.context(|ctx| {
            let main_thread = ctx
                .create_thread(ctx.load(script).into_function().unwrap())
                .unwrap();
            {
                let mut data = self.running_data.lock().unwrap();
                data.add_running_thread(&ctx, main_thread);
            }
        })
    }

    fn env_setup(&mut self) {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            globals
                .set("_yammy_running_coroutine", ctx.create_table().unwrap())
                .unwrap();

            let running_data_cloned = self.running_data.clone();
            let yammy_fork = ctx
                .create_function(move |ctx, function: Function| {
                    let thread = ctx.create_thread(function).unwrap();
                    {
                        let mut data = running_data_cloned.lock().unwrap();
                        data.add_running_thread(&ctx, thread);
                    }
                    Ok(())
                })
                .unwrap();
            globals.set("yammy_fork", yammy_fork).unwrap();
        })
    }

    fn step(&mut self) -> bool {
        let to_run;
        {
            let data = self.running_data.lock().unwrap();
            to_run = data.list_run_possibility();
            if to_run.is_empty() {
                return false;
            }
        }
        for to_run_id in to_run {
            self.continue_running_thread(to_run_id);
        }
        true
    }

    fn continue_running_thread(&mut self, id: u64) {
        self.lua.context(|ctx| {
            let globals = ctx.globals();
            let running_coroutine_table =
                globals.get::<_, Table>("_yammy_running_coroutine").unwrap();
            let thread = running_coroutine_table.get::<_, Thread>(id).unwrap();
            let result = thread.resume::<(), Option<YieldResult>>(()).unwrap();
            // check if the thread is finished
            let mut data = self.running_data.lock().unwrap();
            match thread.status() {
                ThreadStatus::Resumable => match result {
                    None => panic!(),
                    Some(value) => {
                        data.set_running_thread_lock(id, value.lock);
                    }
                }, // The function that yielded managed the state of this
                ThreadStatus::Unresumable => {
                    data.delete_running_thread(id);
                    running_coroutine_table.set(id, Nil).unwrap();
                    globals
                        .set("_yammy_running_coroutine", running_coroutine_table)
                        .unwrap();
                }
                ThreadStatus::Error => panic!(),
            }
        });
    }

    pub fn execute(&mut self) {
        while self.step() {}
    }

    pub fn context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Context) -> R,
    {
        self.lua.context(f)
    }
}

pub fn add_locking_function<'lua>(
    ctx: &Context<'lua>,
    front_user_function_name: &str,
    internal_function_name: &str,
    internal_function: Function<'lua>,
) {
    let globals = ctx.globals();
    globals
        .set(internal_function_name, internal_function)
        .unwrap();
    let front_function = ctx
        .load(&format!(
            "local temp_result = {}()\ncoroutine.yield(temp_result)",
            internal_function_name
        ))
        .into_function()
        .unwrap();
    globals
        .set(front_user_function_name, front_function)
        .unwrap();
}

mod test {
    #[test]
    fn test_running_lua() {
        use crate::RunningLua;
        let mut runninglua = RunningLua::new_from_script("a = 3");
        runninglua.execute();
        runninglua.context(|ctx| {
            let globals = ctx.globals();
            assert_eq!(globals.get::<_, u64>("a").unwrap(), 3);
        })
    }
    #[test]
    fn test_lock() {
        use crate::{add_locking_function, LockReason, RunningLua, YieldResult};
        use std::sync::{
            atomic::{AtomicBool, Ordering::Relaxed},
            Arc,
        };
        let mut runninglua = RunningLua::default();
        let pass_value = Arc::new(AtomicBool::new(false));
        runninglua.context(|ctx| {
            let pass_value_cloned = pass_value.clone();
            let _yammy_test_lock_internal = ctx
                .create_function(move |_, ()| {
                    Ok(YieldResult::new(LockReason::WaitAtomicBool(
                        pass_value_cloned.clone(),
                    )))
                })
                .unwrap();

            add_locking_function(
                &ctx,
                "yammy_test_lock",
                "_yammy_test_lock_internal",
                _yammy_test_lock_internal,
            );
        });

        runninglua.load_script(
            "
            a = 0
            function testa()
                a = a + 1
                yammy_test_lock()
                a = a + 1
            end
            yammy_fork(testa)
            yammy_test_lock()
            a = a + 1",
        );

        for _ in 0..10 {
            runninglua.execute();
        }

        runninglua.context(|ctx| {
            let globals = ctx.globals();
            assert_eq!(globals.get::<_, u64>("a").unwrap(), 1);
        });

        pass_value.store(true, Relaxed);

        for _ in 0..10 {
            runninglua.execute();
        }

        runninglua.context(|ctx| {
            let globals = ctx.globals();
            assert_eq!(globals.get::<_, u64>("a").unwrap(), 3);
        });
    }
}
