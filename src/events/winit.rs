use super::Events;
use super::errors::*;
use gfx::{Gfx, GfxLoopBuilder};
use std::sync::Arc;
use winit;

pub struct WinitEvents {
    events_loop: winit::EventsLoop,
}

impl WinitEvents {
    pub fn new() -> Result<WinitEvents> {
        let events_loop = winit::EventsLoop::new();
        Ok(WinitEvents { events_loop: events_loop })
    }

    /// Setup GFX
    ///
    /// This is here, since the gfx and window bindings need access to the event loop.
    /// Possibly solve with some kind of DI?
    pub fn setup_gfx(&self) -> Result<(Gfx, GfxLoopBuilder)> {
        #[cfg(feature = "gfx-vulkan")]
        {
            use gfx::vulkan;

            let instance = vulkan::VulkanGfxInstance::new()?;
            let window = Arc::new(instance.build_window(&self.events_loop)?);
            let (gfx, gfx_loop_builder) = instance.build_gfx(window)?;

            return Ok((gfx, gfx_loop_builder));
        }

        // statement is only run if no other backends are compiled in.
        #[allow(unreachable_code)]
        {
            return Err(ErrorKind::NoCompatibleGfxBackend.into());
        }
    }

    pub fn poll_events<F>(&mut self, callback: F)
    where
        F: FnMut(winit::Event),
    {
        self.events_loop.poll_events(callback)
    }
}

impl Events for WinitEvents {}
