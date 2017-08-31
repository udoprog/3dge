#[cfg(feature = "gfx-vulkan")]
use super::vulkan::errors as vulkan;

error_chain! {
    foreign_links {
        Image(::image::ImageError);
    }

    links {
        Vulkan(vulkan::Error, vulkan::ErrorKind) #[cfg(feature = "gfx-vulkan")];
    }

    errors {
        PoisonError {
        }

        UnsupportedShader {
        }

        NoCompositeAlphaCapability {
        }

        NoSupportedDevice {
        }

        NoQueueFamily {
        }

        NoQueueAvailable {
        }

        NoSubpass {
        }

        NoWindowDimensions {
        }

        Disconnected {
        }

        SendError {
        }

        MissingPreviousFrame {
        }
    }
}
