use events::errors as events;
use gfx::errors as gfx;

error_chain! {
    foreign_links {
        Gfx(gfx::Error);
        Events(events::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }
}
