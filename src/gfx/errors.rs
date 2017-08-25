use vulkano_win;

pub mod vulkan {
    macro_rules! vulkan_error {
        ( $mod:ident => $name:ident { $( $error:ident; )* } ) => {
            #[derive(Debug)]
            pub enum $name {
                $( $error($mod::$error), )*
            }

            $(
            impl From<$mod::$error> for super::Error {
                fn from(value: $mod::$error) -> super::Error {
                    super::Error::Vulkan(ErrorKind::$name($name::$error(value)))
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
            super::Error::Vulkan(ErrorKind::OomError(value))
        }
    }

    #[derive(Debug)]
    pub enum ErrorKind {
        OomError(::vulkano::OomError),
        NoCompositeAlphaCapability,
        NoSupportedDevice,
        NoQueueFamily,
        NoQueueAvailable,
        NoSubpass,
        NoWindowDimensions,
        CommandBuffer(CommandBuffer),
        Device(Device),
        Framebuffer(Framebuffer),
        Instance(Instance),
        Memory(Memory),
        Pipeline(Pipeline),
        Swapchain(Swapchain),
        Sync(Sync),
        DescriptorSet(DescriptorSet),
        CpuAccess(CpuAccess),
    }

    // result type should have gfx errors, since the backend mostly interfaces with the gfx stack.
    pub type Result<T> = ::std::result::Result<T, super::Error>;
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
    Vulkan(vulkan::ErrorKind),
    VulkanoWin(VulkanoWinError),
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<vulkan::ErrorKind> for Error {
    fn from(value: vulkan::ErrorKind) -> Error {
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
