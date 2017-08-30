use super::errors::*;
use super::fps_counter::FpsCounter;
use gfx::GfxLoopBuilder;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

pub struct GfxThread {
    stopped: Arc<AtomicBool>,
    errored: Arc<AtomicBool>,
    enabled: Arc<(Mutex<bool>, Condvar)>,
    handle: Option<thread::JoinHandle<Result<()>>>,
}

impl GfxThread {
    pub fn new() -> GfxThread {
        GfxThread {
            stopped: Arc::new(AtomicBool::new(false)),
            errored: Arc::new(AtomicBool::new(false)),
            enabled: Arc::new((Mutex::new(false), Condvar::new())),
            handle: None,
        }
    }

    /// Indicates if graphics should be updating or not.
    ///
    /// This is typically used to disable rendering while a window is not focused.
    pub fn enabled(&self, state: bool) -> Result<()> {
        let mut guard = self.enabled.0.lock().map_err(|_| ErrorKind::PoisonError)?;
        *guard = state;
        self.enabled.1.notify_all();
        Ok(())
    }

    pub fn errored(&self) -> bool {
        self.errored.load(Ordering::Relaxed)
    }

    pub fn stop(&mut self) -> Result<()> {
        let cas = self.stopped.compare_and_swap(
            false,
            true,
            Ordering::Relaxed,
        );

        if cas {
            if let Some(handle) = self.handle.take() {
                return handle.join().map_err(|_| ErrorKind::ThreadJoin)?;
            }
        }

        Ok(())
    }

    pub fn start(&mut self, gfx_loop_builder: GfxLoopBuilder) -> Result<()> {
        let errored = self.errored.clone();
        let stopped = self.stopped.clone();
        let enabled = self.enabled.clone();

        self.handle = Some(thread::spawn(move || {
            let mut gfx_loop = gfx_loop_builder.into_loop()?;

            let mut fps_counter = FpsCounter::new(|fps| {
                info!("fps = {}", fps);
                Ok(())
            });

            let (ref enabled_mutex, ref enabled_cond) = *enabled;

            while !stopped.load(Ordering::Relaxed) {
                {
                    let mut guard = enabled_mutex.lock().map_err(|_| ErrorKind::PoisonError)?;

                    if !*guard {
                        while !*guard {
                            guard = enabled_cond.wait(guard).map_err(|_| ErrorKind::PoisonError)?;
                        }

                        fps_counter.reset()?;
                    }
                }

                if let Err(e) = gfx_loop.tick() {
                    error!("error in gfx thread: {:?}", e);
                    errored.store(true, Ordering::Relaxed);
                    return Err(e.into());
                }

                fps_counter.tick()?;
            }

            Ok(())
        }));

        Ok(())
    }
}
