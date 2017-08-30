use super::boxed_scene::BoxedScene;
use super::core_state::CoreState;
use super::errors::*;
use super::events::winit::WinitEvents;
use super::gfx_thread::GfxThread;
use super::into_boxed_scene::IntoBoxedScene;
use super::pressed_keys::PressedKeys;
use super::scheduler::Scheduler;
use cgmath::{Matrix4, SquareMatrix, Vector3};
use cgmath::prelude::*;
use gfx::{Gfx, GfxLoopBuilder};
use shuteye;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

pub struct CoreLoop {
    gfx: Box<Gfx>,
    gfx_loop_builder: Box<GfxLoopBuilder>,
    core: Rc<RefCell<CoreState>>,
    core_scheduler: Scheduler<Rc<RefCell<CoreState>>>,
    scene: Option<Box<BoxedScene<CoreState>>>,
}

impl CoreLoop {
    pub fn new() -> Result<CoreLoop> {
        let events = WinitEvents::new()?;
        let (gfx, gfx_loop_builder) = events.setup_gfx()?;

        Ok(CoreLoop {
            gfx: gfx.clone(),
            gfx_loop_builder: gfx_loop_builder,
            core: Rc::new(RefCell::new(CoreState {
                no_transform: <Matrix4<f32> as SquareMatrix>::identity(),
                no_movement: Vector3::zero(),
                pressed_keys: PressedKeys::new(),
                focus_update: None,
                focused: true,
                exit: false,
                scroll: 0i32,
                gfx: gfx.clone(),
                gfx_thread: GfxThread::new(),
                events: events,
            })),
            core_scheduler: Scheduler::new(),
            scene: None,
        })
    }

    /// Set the startup scene.
    pub fn set_scene<S: IntoBoxedScene<CoreState>>(&mut self, scene: S) -> Result<()> {
        self.scene = Some(scene.into_boxed_scene(self.gfx.clone())?);
        Ok(())
    }

    pub fn run(mut self) -> Result<()> {
        let gfx_loop_builder = self.gfx_loop_builder;

        self.core.try_borrow_mut()?.deref_mut().gfx_thread.start(
            gfx_loop_builder,
        )?;

        self.core_scheduler.on_every_tick(Box::new(|_, core| {
            let mut core = core.try_borrow_mut()?;
            let c = core.deref_mut();

            if let Some(state) = c.focus_update.take() {
                c.gfx_thread.enabled(state)?;
                c.focused = state;
            }

            Ok(())
        }));

        self.core_scheduler.on_every_tick(Box::new(|_, core| {
            core.try_borrow_mut()?.update_pressed_keys();
            Ok(())
        }));

        let target_sleep = Duration::from_millis(10);
        let mut sleep = target_sleep;

        loop {
            if self.core.try_borrow()?.gfx_thread.errored() {
                error!("exiting due to error in gfx thread");
                break;
            }

            shuteye::sleep(sleep);

            let before = Instant::now();

            self.core_scheduler.tick(&mut self.core)?;

            if let Some(ref mut scene) = self.scene {
                scene.tick(self.core.clone())?;
            }

            let elapsed = before.elapsed();

            if let Some(s) = target_sleep.checked_sub(elapsed) {
                sleep = s;
            } else {
                warn!(
                    "frame took longer ({:?}) than the desired frame length ({:?}) to execute",
                    elapsed,
                    target_sleep
                );
            }

            if self.core.try_borrow()?.exit {
                break;
            }
        }

        self.core.try_borrow_mut()?.gfx_thread.stop()?;
        Ok(())
    }
}
