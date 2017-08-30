macro_rules! vulkan_link_error {
    ($name:ty) => {
    impl From<$name> for $crate::gfx::errors::Error {
        fn from(value: $name) -> $crate::gfx::errors::Error {
            let error: self::Error = value.into();
            let error: $crate::gfx::vulkan::errors::Error = error.into();
            error.into()
        }
    }
    }
}

mod command_buffer {
    use vulkano::command_buffer::*;

    error_chain! {
        foreign_links {
            AutoCommandBufferBuilderContextError(AutoCommandBufferBuilderContextError);
            BuildError(BuildError);
            DrawError(DrawError);
            DrawIndexedError(DrawIndexedError);
            BeginRenderPassError(BeginRenderPassError);
            CommandBufferExecError(CommandBufferExecError);
        }
    }

    vulkan_link_error!(AutoCommandBufferBuilderContextError);
    vulkan_link_error!(BuildError);
    vulkan_link_error!(DrawError);
    vulkan_link_error!(DrawIndexedError);
    vulkan_link_error!(BeginRenderPassError);
    vulkan_link_error!(CommandBufferExecError);
}

mod device {
    use vulkano::device::*;

    error_chain! {
        foreign_links {
            DeviceCreationError(DeviceCreationError);
        }
    }

    vulkan_link_error!(DeviceCreationError);
}

mod memory {
    use vulkano::memory::*;

    error_chain! {
        foreign_links {
            DeviceMemoryAllocError(DeviceMemoryAllocError);
        }
    }

    vulkan_link_error!(DeviceMemoryAllocError);
}

mod instance {
    use vulkano::instance::*;

    error_chain! {
        foreign_links {
            InstanceCreationError(InstanceCreationError);
        }
    }

    vulkan_link_error!(InstanceCreationError);
}

mod sync {
    use vulkano::sync::*;

    error_chain! {
        foreign_links {
            FlushError(FlushError);
        }
    }

    vulkan_link_error!(FlushError);
}

mod swapchain {
    use vulkano::swapchain::*;

    error_chain! {
        foreign_links {
            SwapchainCreationError(SwapchainCreationError);
            CapabilitiesError(CapabilitiesError);
            AcquireError(AcquireError);
        }
    }

    vulkan_link_error!(SwapchainCreationError);
    vulkan_link_error!(CapabilitiesError);
    vulkan_link_error!(AcquireError);
}

mod pipeline {
    use vulkano::pipeline::*;

    error_chain! {
        foreign_links {
            GraphicsPipelineCreationError(GraphicsPipelineCreationError);
        }
    }

    vulkan_link_error!(GraphicsPipelineCreationError);
}

mod framebuffer {
    use vulkano::framebuffer::*;

    error_chain! {
        foreign_links {
            RenderPassCreationError(RenderPassCreationError);
            FramebufferCreationError(FramebufferCreationError);
        }
    }

    vulkan_link_error!(RenderPassCreationError);
    vulkan_link_error!(FramebufferCreationError);
}

mod descriptor_set {
    use vulkano::descriptor::descriptor_set::*;

    error_chain! {
        foreign_links {
            PersistentDescriptorSetError(PersistentDescriptorSetError);
            PersistentDescriptorSetBuildError(PersistentDescriptorSetBuildError);
        }
    }

    vulkan_link_error!(PersistentDescriptorSetError);
    vulkan_link_error!(PersistentDescriptorSetBuildError);
}

mod cpu_access {
    use vulkano::buffer::cpu_access::*;

    error_chain! {
        foreign_links {
            WriteLockError(WriteLockError);
        }
    }

    vulkan_link_error!(WriteLockError);
}

mod image {
    use vulkano::image::*;

    error_chain! {
        foreign_links {
            ImageCreationError(ImageCreationError);
        }
    }

    vulkan_link_error!(ImageCreationError);
}

mod vulkano_win {
    use vulkano_win::*;

    error_chain! {
        foreign_links {
            CreationError(CreationError);
        }
    }

    vulkan_link_error!(CreationError);
}

use gfx::errors as gfx;

error_chain! {
    foreign_links {
        OomError(::vulkano::OomError);
    }

    links {
        VulkanoWin(self::vulkano_win::Error, self::vulkano_win::ErrorKind);
        CommandBuffer(self::command_buffer::Error, self::command_buffer::ErrorKind);
        Device(self::device::Error, self::device::ErrorKind);
        Framebuffer(self::framebuffer::Error, self::framebuffer::ErrorKind);
        Instance(self::instance::Error, self::instance::ErrorKind);
        Memory(self::memory::Error, self::memory::ErrorKind);
        Pipeline(self::pipeline::Error, self::pipeline::ErrorKind);
        Swapchain(self::swapchain::Error, self::swapchain::ErrorKind);
        Sync(self::sync::Error, self::sync::ErrorKind);
        DescriptorSet(self::descriptor_set::Error, self::descriptor_set::ErrorKind);
        CpuAccess(self::cpu_access::Error, self::cpu_access::ErrorKind);
        Image(self::image::Error, self::image::ErrorKind);
    }
}

impl From<::vulkano::OomError> for gfx::Error {
    fn from(value: ::vulkano::OomError) -> gfx::Error {
        let error: Error = value.into();
        error.into()
    }
}
