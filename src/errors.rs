use events::errors as events;
use gfx;
use texture::errors as texture;

error_chain! {
    foreign_links {
        BorrowMutError(::std::cell::BorrowMutError);
        BorrowError(::std::cell::BorrowError);
        IoError(::std::io::Error);
        Gltf(::gltf::Error);
        Events(events::Error);
        Texture(texture::Error);
        SystemTimeError(::std::time::SystemTimeError);
    }

    links {
        Gfx(gfx::errors::Error, gfx::errors::ErrorKind);
        Vulkan(gfx::vulkan::errors::Error, gfx::vulkan::errors::ErrorKind) #[cfg(feature = "gfx-vulkan")];
    }

    errors {
        ThreadJoin {
        }

        PoisonError {
        }
    }
}
