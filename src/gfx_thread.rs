use super::errors::*;
use super::fps_counter::FpsCounter;
use gfx::Gfx;
use gfx::camera_geometry::CameraGeometry;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;

pub struct GfxThread {
    stopped: Arc<AtomicBool>,
    enabled: Arc<(Mutex<bool>, Condvar)>,
    gfx: Box<Gfx>,
    handle: Option<thread::JoinHandle<Result<()>>>,
}

impl GfxThread {
    pub fn new(gfx: Box<Gfx>) -> GfxThread {
        GfxThread {
            stopped: Arc::new(AtomicBool::new(false)),
            enabled: Arc::new((Mutex::new(false), Condvar::new())),
            gfx: gfx,
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

    pub fn start(&mut self, camera: Box<CameraGeometry>) {
        let gfx = self.gfx.clone();
        let stopped = self.stopped.clone();
        let enabled = self.enabled.clone();

        self.handle = Some(thread::spawn(move || {
            let mut fps_counter = FpsCounter::new(|fps| {
                println!("fps = {}", fps);
                Ok(())
            });

            let (ref enabled_mutex, ref enabled_cond) = *enabled;

            let mut gfx_loop = gfx.new_loop(camera)?;

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

                gfx_loop.tick()?;
                fps_counter.tick()?;
            }

            Ok(())
        }));
    }
}