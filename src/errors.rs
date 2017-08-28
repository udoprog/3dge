use events::errors as events;
use gfx::errors as gfx;
use texture::errors as texture;

error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        Gltf(::gltf::Error);
        Gfx(gfx::Error);
        Events(events::Error);
        Texture(texture::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }

    errors {
        ThreadJoin {
        }

        PoisonError {
        }
    }
}
