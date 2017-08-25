use super::errors::*;
use std::time::{Duration, SystemTime};

pub struct FpsCounter<C> {
    one_sec: Duration,
    start: SystemTime,
    frames: u64,
    callback: C,
}

impl<C> FpsCounter<C>
where
    C: Fn(u64) -> Result<()>,
{
    pub fn new(callback: C) -> FpsCounter<C> {
        FpsCounter {
            one_sec: Duration::from_secs(1),
            start: SystemTime::now(),
            frames: 0u64,
            callback: callback,
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        self.frames += 1;
        let now = SystemTime::now();

        if now.duration_since(self.start)? > self.one_sec {
            (&mut self.callback)(self.frames)?;
            self.reset()?;
        }

        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.start = SystemTime::now();
        self.frames = 0;
        Ok(())
    }
}
