#[cfg(feature = "gfx-vulkan")]
use super::vulkan::errors as vulkan;

error_chain! {
    foreign_links {
        SystemTimeError(::std::time::SystemTimeError);
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
    }
}
