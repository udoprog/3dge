use super::errors::*;
use std::collections::HashMap;
use std::mem;

/// Every callback gets exclusive access to the scheduler, permitting them to modify subsequent scheduling.
pub type CallbackFn<S> = Fn(&mut SelfScheduler<S>, &mut S) -> Result<()>;

pub trait SchedulerSetup<S> {
    fn setup_scheduler(&mut self, &mut Scheduler<S>);
}

pub trait SelfScheduler<S> {
    /// Run the given task at the given tick offset.
    fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>);

    /// Run on every tick, from now until eternity.
    fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>);

    /// Re-schedule self.
    fn run_self_at(&mut self, tick_offset: u32);
}

enum Task<S> {
    RunAt {
        tick_offset: u32,
        callback: Box<CallbackFn<S>>,
    },
    OnEveryTick { callback: Box<CallbackFn<S>> },
}

pub enum SelfTask {
    RunAt { tick_offset: u32 },
}

pub struct Scheduler<S> {
    current_tick: u64,
    on_tick: HashMap<u64, Vec<Box<CallbackFn<S>>>>,
    on_every_tick: Vec<Box<CallbackFn<S>>>,
}

impl<S> Scheduler<S> {
    pub fn new() -> Scheduler<S> {
        Scheduler {
            current_tick: 0u64,
            on_tick: HashMap::new(),
            on_every_tick: Vec::new(),
        }
    }

    pub fn tick(&mut self, state: &mut S) -> Result<()> {
        let current_tick = self.current_tick;

        let mut new_tasks = Vec::new();

        // run the things that happen on every tick
        for task in &self.on_every_tick {
            let mut internal = (&mut new_tasks, None);
            task(&mut internal, state)?;
        }

        if let Some(tasks) = self.on_tick.remove(&current_tick) {
            for task in tasks {
                let mut internal = (&mut new_tasks, None);

                task(&mut internal, state)?;

                // Current task has requested to be re-scheduled.
                if let Some(self_task) = internal.1 {
                    match self_task {
                        SelfTask::RunAt { tick_offset } => {
                            self.on_tick
                                .entry(current_tick + tick_offset as u64)
                                .or_insert_with(Vec::new)
                                .push(task);
                        }
                    }
                }
            }
        }

        for new_task in new_tasks {
            use self::Task::*;

            match new_task {
                OnEveryTick { callback } => {
                    self.on_every_tick.push(callback);
                }
                RunAt {
                    callback,
                    tick_offset,
                } => {
                    self.on_tick
                        .entry(current_tick + tick_offset as u64)
                        .or_insert_with(Vec::new)
                        .push(callback);
                }
            }
        }

        self.current_tick = current_tick + 1;
        Ok(())
    }

    pub fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>) {
        self.on_tick
            .entry(self.current_tick + tick_offset as u64)
            .or_insert_with(Vec::new)
            .push(callback);
    }

    pub fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>) {
        self.on_every_tick.push(callback);
    }
}

impl<'a, S> SelfScheduler<S> for (&'a mut Vec<Task<S>>, Option<SelfTask>) {
    fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>) {
        self.0.push(Task::RunAt {
            tick_offset: tick_offset,
            callback: callback,
        });
    }

    fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>) {
        self.0.push(Task::OnEveryTick { callback: callback });
    }

    fn run_self_at(&mut self, tick_offset: u32) {
        mem::replace(
            &mut self.1,
            Some(SelfTask::RunAt { tick_offset: tick_offset }),
        );
    }
}
