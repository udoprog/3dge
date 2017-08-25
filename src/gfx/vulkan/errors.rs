use gfx::errors as gfx;

use vulkano_win;

macro_rules! vulkan_error {
    ( $mod:ident => $name:ident { $( $error:ident; )* } ) => {
        #[derive(Debug)]
        pub enum $name {
            $( $error($mod::$error), )*
        }

        $(
        impl From<$mod::$error> for gfx::Error {
            fn from(value: $mod::$error) -> gfx::Error {
                gfx::Error::Vulkan(ErrorKind::$name($name::$error(value)))
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

impl From<OomError> for gfx::Error {
    fn from(value: OomError) -> gfx::Error {
        gfx::Error::Vulkan(ErrorKind::OomError(value))
    }
}

vulkan_error!{
    vulkano_win => VulkanoWin {
        CreationError;
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
    VulkanoWin(VulkanoWin),
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

#[derive(Debug)]
pub enum VulkanoWinError {
    CreationError(vulkano_win::CreationError),
    NoDimensions,
}

// result type should have gfx errors, since the backend mostly interfaces with the gfx stack.
pub type Result<T> = ::std::result::Result<T, gfx::Error>;
