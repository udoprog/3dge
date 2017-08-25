use super::Events;
use super::errors::*;
use gfx;
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
    pub fn setup_gfx(&self) -> Result<(Arc<Box<gfx::Window>>, Box<gfx::Gfx>)> {
        #[cfg(feature = "gfx-vulkan")]
        {
            let instance = gfx::vulkan::VulkanGfxInstance::new()?;
            let window = instance.build_window(&self.events_loop)?;
            let gfx = instance.build_gfx(&window)?;

            return Ok((Arc::new(Box::new(window)), Box::new(gfx)));
        }

        // statement is only run if no other backends are compiled in.
        #[allow(unreachable_code)]
        {
            return Err(Error::NoCompatibleGfxBackend);
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
