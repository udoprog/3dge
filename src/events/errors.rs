use gfx::errors as gfx;
use gfx::vulkan::errors as vulkan;

error_chain! {
    foreign_links {
        Gfx(gfx::Error);
    }

    links {
        Vulkan(vulkan::Error, vulkan::ErrorKind);
    }

    errors {
        NoCompatibleGfxBackend {
        }
    }
}
