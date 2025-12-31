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

    let hwnd = window.get_win32_window() as isize;
    let create_surface_info = Win32SurfaceCreateInfoKHR {
        hwnd,
        ..Default::default()
    };

    let win32_surface_instance = win32_surface::Instance::new(&vulkan_entry, &vulkan_instance);
    let surface =
        unsafe { win32_surface_instance.create_win32_surface(&create_surface_info, None) }
            .expect("failed to create win32 surface");

    let surface_instance = surface::Instance::new(&vulkan_entry, &vulkan_instance);

    let physical_devices = unsafe { vulkan_instance.enumerate_physical_devices() }
        .expect("failed to get physical devices with Vulkan support");
    if physical_devices.is_empty() {
        println!("None GPUs have Vulkan support");
        exit(-1)
    }

    let required_device_extensions = [vk::KHR_SWAPCHAIN_NAME];

    let selected_device_and_queues = select_device_and_queue(
        &vulkan_instance,
        &physical_devices,
        &surface_instance,
        surface,
        &required_device_extensions,
    )
    .expect("failed to find suitable device and queues");

    let graphics_queue_create_info = DeviceQueueCreateInfo {
        queue_family_index: selected_device_and_queues.graphics_family_queue_index,
        p_queue_priorities: &1.0f32,
        queue_count: 1,
        ..Default::default()
    };

    let present_queue_create_info = DeviceQueueCreateInfo {
        queue_family_index: selected_device_and_queues.present_family_queue_index,
        p_queue_priorities: &1.0f32,
        queue_count: 1,
        ..Default::default()
    };

    let mut queue_create_infos = Vec::with_capacity(2);

    if selected_device_and_queues.graphics_family_queue_index
        == selected_device_and_queues.present_family_queue_index
    {
        let queue_create_info = DeviceQueueCreateInfo {
            queue_family_index: selected_device_and_queues.graphics_family_queue_index,
            p_queue_priorities: &1.0f32,
            queue_count: 1,
            ..Default::default()
        };
        queue_create_infos.push(queue_create_info)
    } else {
        let graphics_queue_create_info = DeviceQueueCreateInfo {
            queue_family_index: selected_device_and_queues.graphics_family_queue_index,
            p_queue_priorities: &1.0f32,
            queue_count: 1,
            ..Default::default()
        };
        queue_create_infos.push(graphics_queue_create_info);

        let present_queue_create_info = DeviceQueueCreateInfo {
            queue_family_index: selected_device_and_queues.present_family_queue_index,
            p_queue_priorities: &1.0f32,
            queue_count: 1,
            ..Default::default()
        };
        queue_create_infos.push(present_queue_create_info)
    }

    let device_create_info = DeviceCreateInfo {
        p_queue_create_infos: queue_create_infos.as_ptr(),
        queue_create_info_count: queue_create_infos.len() as u32,
        enabled_extension_count: required_device_extensions.len() as u32,
        pp_enabled_extension_names: required_device_extensions.as_ptr() as *const *const i8,
        ..Default::default()
    };

    let logical_device = unsafe {
        vulkan_instance.create_device(
            selected_device_and_queues.physical_device,
            &device_create_info,
            None,
        )
    }
    .expect("failed to create Vulkan logical device");

    println!("{:?}", selected_device_and_queues);

    let graphics_queue = unsafe {
        logical_device.get_device_queue(selected_device_and_queues.graphics_family_queue_index, 0)
    };
    let present_queue = unsafe {
        logical_device.get_device_queue(selected_device_and_queues.present_family_queue_index, 0)
    };

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

#[derive(Debug)]
struct DeviceAndQueue {
    physical_device: vk::PhysicalDevice,
    graphics_family_queue_index: u32,
    present_family_queue_index: u32,
}

fn select_device_and_queue(
    vulkan_instance: &ash::Instance,
    physical_devices: &[vk::PhysicalDevice],
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
    required_device_extensions: &[&CStr],
) -> Option<DeviceAndQueue> {
    let mut device_and_queues_selected = None;

    println!("Available GPUs with Vulkan support:");
    for physical_device in physical_devices.iter().cloned() {
        let device_properties =
            unsafe { vulkan_instance.get_physical_device_properties(physical_device) };
        let device_name =
            unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) }.to_string_lossy();
        let device_id = device_properties.device_id;

        let physical_device_extensions =
            unsafe { vulkan_instance.enumerate_device_extension_properties(physical_device) }
                .expect("failed to get extension supported by physical device");

        let mut all_extension_found = true;
        for required_device_extension in required_device_extensions {
            let mut extension_found = false;
            for extension in &physical_device_extensions {
                if extension
                    .extension_name_as_c_str()
                    .expect("wrong extension name format(try to use UTF-8)")
                    == required_device_extension
                {
                    extension_found = true;
                    break;
                }
            }
            if !extension_found {
                all_extension_found = false;
                break;
            }
        }

        if !all_extension_found {
            continue;
        }

        println!("- {}(id: {})", device_name, device_id);
        println!("  With available family queue:");

        let queues_family_properties =
            unsafe { vulkan_instance.get_physical_device_queue_family_properties(physical_device) };

        let mut selected_graphics_queue_family_index_opt = None;
        let mut selected_present_queue_family_index_opt = None;

        for (queue_family_index, queue_family_properties) in
            queues_family_properties.into_iter().enumerate()
        {
            if selected_graphics_queue_family_index_opt.is_none()
                && queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            {
                selected_graphics_queue_family_index_opt = Some(queue_family_index as u32)
            }
            if selected_present_queue_family_index_opt.is_none()
                && unsafe {
                    surface_instance.get_physical_device_surface_support(
                        physical_device,
                        queue_family_index as u32,
                        surface,
                    )
                }
                .expect("failed to get surface support of physical device queue")
            {
                selected_present_queue_family_index_opt = Some(queue_family_index as u32)
            }

            println!(
                "  - {} queue: {:?}",
                queue_family_properties.queue_count, queue_family_properties.queue_flags
            );
        }

        if let Some(selected_graphics_queue_index) = selected_graphics_queue_family_index_opt
            && let Some(selected_present_queue_index) = selected_present_queue_family_index_opt
        {
            device_and_queues_selected = Some(DeviceAndQueue {
                physical_device,
                graphics_family_queue_index: selected_graphics_queue_index,
                present_family_queue_index: selected_present_queue_index,
            });
        }
    }
    println!();

    device_and_queues_selected
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
