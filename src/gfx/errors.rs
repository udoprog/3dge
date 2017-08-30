error_chain! {
    links {
        Vulkan(super::vulkan::errors::Error, super::vulkan::errors::ErrorKind) #[cfg(feature = "gfx-vulkan")];
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
    }
}
