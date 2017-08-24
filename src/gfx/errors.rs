use vulkano_win;

mod vulkan_errors {
    macro_rules! vulkan_error {
        ( $mod:ident => $name:ident { $( $error:ident; )* } ) => {
            #[derive(Debug)]
            pub enum $name {
                $( $error($mod::$error), )*
            }

            $(
            impl From<$mod::$error> for super::Error {
                fn from(value: $mod::$error) -> super::Error {
                    super::Error::Vulkan(super::VulkanError::$name($name::$error(value)))
                }
            }
            )*
        };
    }

    use vulkano::command_buffer;

    vulkan_error!{
        command_buffer => CommandBuffer {
            AutoCommandBufferBuilderContextError;
            BuildError;
            DrawError;
            BeginRenderPassError;
            CommandBufferExecError;
        }
    }

    use vulkano::device;

    vulkan_error!{
        device => Device {
            DeviceCreationError;
        }
    }

    use vulkano::memory;

    vulkan_error!{
        memory => Memory {
            DeviceMemoryAllocError;
        }
    }

    use vulkano::instance;

    vulkan_error!{
        instance => Instance {
            InstanceCreationError;
        }
    }

    use vulkano::sync;

    vulkan_error!{
        sync => Sync {
            FlushError;
        }
    }

    use vulkano::swapchain;

    vulkan_error!{
        swapchain => Swapchain {
            SwapchainCreationError;
            CapabilitiesError;
        }
    }

    use vulkano::pipeline;

    vulkan_error!{
        pipeline => Pipeline {
            GraphicsPipelineCreationError;
        }
    }

    use vulkano::framebuffer;

    vulkan_error!{
        framebuffer => Framebuffer {
            RenderPassCreationError;
            FramebufferCreationError;
        }
    }

    use vulkano::descriptor::descriptor_set;

    vulkan_error!{
        descriptor_set => DescriptorSet {
          PersistentDescriptorSetError;
          PersistentDescriptorSetBuildError;
        }
    }

    use vulkano::buffer::cpu_access;

    vulkan_error!{
        cpu_access => CpuAccess {
            WriteLockError;
        }
    }

    use vulkano::OomError;

    impl From<OomError> for super::Error {
        fn from(value: OomError) -> super::Error {
            super::Error::Vulkan(super::VulkanError::OomError(value))
        }
    }
}

#[derive(Debug)]
pub enum VulkanError {
    OomError(::vulkano::OomError),
    NoCompositeAlphaCapability,
    NoSupportedDevice,
    NoQueueFamily,
    NoQueueAvailable,
    NoSubpass,
    CommandBuffer(self::vulkan_errors::CommandBuffer),
    Device(self::vulkan_errors::Device),
    Framebuffer(self::vulkan_errors::Framebuffer),
    Instance(self::vulkan_errors::Instance),
    Memory(self::vulkan_errors::Memory),
    Pipeline(self::vulkan_errors::Pipeline),
    Swapchain(self::vulkan_errors::Swapchain),
    Sync(self::vulkan_errors::Sync),
    DescriptorSet(self::vulkan_errors::DescriptorSet),
    CpuAccess(self::vulkan_errors::CpuAccess),
}

#[derive(Debug)]
pub enum VulkanoWinError {
    CreationError(vulkano_win::CreationError),
    NoDimensions,
}

#[derive(Debug)]
pub enum Error {
    /// Returned if a backend does not support the given shader.
    UnsupportedShader,
    Vulkan(VulkanError),
    VulkanoWin(VulkanoWinError),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<VulkanError> for Error {
    fn from(value: VulkanError) -> Error {
        Error::Vulkan(value)
    }
}

impl From<VulkanoWinError> for Error {
    fn from(value: VulkanoWinError) -> Error {
        Error::VulkanoWin(value)
    }
}

impl From<vulkano_win::CreationError> for Error {
    fn from(value: vulkano_win::CreationError) -> Error {
        Error::VulkanoWin(VulkanoWinError::CreationError(value))
    }
}
