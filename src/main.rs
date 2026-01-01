use crate::{app::Application, glfw::GlfwEntry, vulkan::entry::VulkanEntry};

mod app;
mod glfw;
mod vulkan;

fn main() {
    let mut glfw_entry = GlfwEntry::init();
    let vulkan_entry = VulkanEntry::init();

    let app = Application::new(&mut glfw_entry, &vulkan_entry, "u3DEngine(Vulkan)");

    app.run();
}
/*

    let surface_capabilities = unsafe {
        surface_instance.get_physical_device_surface_capabilities(
            selected_device_and_queues.physical_device,
            surface,
        )
    }
    .expect("failed to get physical device surface capabilities");

    println!("{:?}", surface_capabilities);

    let all_surface_formats = unsafe {
        surface_instance.get_physical_device_surface_formats(
            selected_device_and_queues.physical_device,
            surface,
        )
    }
    .expect("failed to get physical device surface format");

    let all_present_modes = unsafe {
        surface_instance.get_physical_device_surface_present_modes(
            selected_device_and_queues.physical_device,
            surface,
        )
    }
    .expect("failed to get physical device surface present modes");

    let surface_extend2d = select_suitable_surface_extend2d(surface_capabilities);
    let surface_format = select_suitable_surface_format(all_surface_formats);
    let surface_present_mode = select_suitable_surface_present_mode(all_present_modes);

    let mut swapchain_create_info = SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        surface,
        min_image_count: 2,
        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent: surface_extend2d,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        pre_transform: surface_capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: surface_present_mode,
        clipped: vk::TRUE,
        ..Default::default()
    };

    let p_queue_family_indices = [
        selected_device_and_queues.graphics_family_queue_index,
        selected_device_and_queues.present_family_queue_index,
    ];

    if selected_device_and_queues.graphics_family_queue_index
        == selected_device_and_queues.present_family_queue_index
    {
        swapchain_create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
    } else {
        swapchain_create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
        swapchain_create_info.queue_family_index_count = 2;
        swapchain_create_info.p_queue_family_indices = p_queue_family_indices.as_ptr();
    }

    let swapchain_device = swapchain::Device::new(&vulkan_instance, &logical_device);

    let swapchain = unsafe { swapchain_device.create_swapchain(&swapchain_create_info, None) }
        .expect("failed to create Vulkan swapchain");

    let frag_shader_module = load_shader_module(&logical_device, "base.frag.spv");
    let vert_shader_module = load_shader_module(&logical_device, "base.vert.spv");

    const SHADER_ENTRYPOINT: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };

    let vert_shader_stage_create_info = PipelineShaderStageCreateInfo::default()
        .stage(ShaderStageFlags::VERTEX)
        .module(vert_shader_module)
        .name(SHADER_ENTRYPOINT);
    let frag_shader_stage_create_info = PipelineShaderStageCreateInfo::default()
        .stage(ShaderStageFlags::FRAGMENT)
        .module(frag_shader_module)
        .name(SHADER_ENTRYPOINT);

    while !window.should_close() {
        glfw.poll_events();
    }

    unsafe {
        swapchain_device.destroy_swapchain(swapchain, None);
        surface_instance.destroy_surface(surface, None);
        vulkan_instance.destroy_instance(None);
    }

    /*unsafe {
        instance.destroy_instance(None);
    }*/
}

fn select_suitable_surface_extend2d(surface_capabilities: SurfaceCapabilitiesKHR) -> vk::Extent2D {
    vk::Extent2D {
        width: 1920,
        height: 1080,
    }
}

fn select_suitable_surface_format(
    all_surface_formats: Vec<SurfaceFormatKHR>,
) -> vk::SurfaceFormatKHR {
    if all_surface_formats.contains(&vk::SurfaceFormatKHR {
        format: vk::Format::B8G8R8A8_SRGB,
        color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
    }) {
        vk::SurfaceFormatKHR {
            format: vk::Format::B8G8R8A8_SRGB,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        }
    } else {
        all_surface_formats[0] //guaranteed to be available
    }
}

fn select_suitable_surface_present_mode(all_present_modes: Vec<PresentModeKHR>) -> PresentModeKHR {
    if all_present_modes.contains(&PresentModeKHR::MAILBOX) {
        PresentModeKHR::MAILBOX
    } else {
        PresentModeKHR::FIFO //guaranteed to be available
    }
}

fn load_shader_module(logical_device: &ash::Device, name: &str) -> ShaderModule {
    let mut shader_file = File::open("shaders/spir-v/".to_owned() + name).unwrap();
    let mut shader_file_buf = Vec::new();
    shader_file.read_to_end(&mut shader_file_buf).unwrap();

    let shader_code: &[u32] = bytemuck::cast_slice(shader_file_buf.as_slice());

    let shader_module_create_info = ShaderModuleCreateInfo::default().code(shader_code);
    unsafe { logical_device.create_shader_module(&shader_module_create_info, None) }
        .expect("failed to create shader module")
}
*/
