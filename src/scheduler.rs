use super::errors::*;
use std::collections::HashMap;

/// Every callback gets exclusive access to the scheduler, permitting them to modify subsequent scheduling.
pub type CallbackFn<S> = Fn(&mut SchedulerInterface<S>, &mut S) -> Result<()>;

enum Task<S> {
    RunAt {
        tick_offset: u32,
        callback: Box<CallbackFn<S>>,
    },
    OnEveryTick { callback: Box<CallbackFn<S>> },
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
            task(&mut new_tasks, state)?;
        }

        if let Some(tasks) = self.on_tick.remove(&current_tick) {
            for task in tasks {
                task(&mut new_tasks, state)?;
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
}

pub trait SchedulerInterface<S> {
    /// Run the given task at the given tick offset.
    fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>);

    /// Run on every tick, from now until eternity.
    fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>);
}

impl<S> SchedulerInterface<S> for Scheduler<S> {
    fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>) {
        self.on_tick
            .entry(self.current_tick + tick_offset as u64)
            .or_insert_with(Vec::new)
            .push(callback);
    }

    fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>) {
        self.on_every_tick.push(callback);
    }
}

impl<S> SchedulerInterface<S> for Vec<Task<S>> {
    fn run_at(&mut self, tick_offset: u32, callback: Box<CallbackFn<S>>) {
        self.push(Task::RunAt {
            tick_offset: tick_offset,
            callback: callback,
        });
    }

    fn on_every_tick(&mut self, callback: Box<CallbackFn<S>>) {
        self.push(Task::OnEveryTick { callback: callback });
    }
}
