use std::{thread::sleep, time::Duration};

use ash::vk::{self, Extent2D, Format};
use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};

use crate::{
    glfw::GlfwEntry,
    vulkan::{
        device::VulkanDevice, entry::VulkanEntry, graphics_pipeline::VulkanGraphicsPipeline,
        instance::VulkanInstance, shaders::VulkanShaderModule, surface::VulkanSurface,
        swapchain::VulkanSwapchain,
    },
};

pub struct Application {
    glfw_window: PWindow,
    glfw_events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Application {
    pub fn new(glfw_entry: &mut GlfwEntry, vulkan_entry: &VulkanEntry, title: &str) -> Self {
        glfw_entry.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        glfw_entry.window_hint(glfw::WindowHint::CenterCursor(false));
        glfw_entry.window_hint(glfw::WindowHint::AutoIconify(false));

        let (window, events) = glfw_entry
            .with_connected_monitors(|glfw_entry, monitors| {
                if monitors.len() > 1 {
                    let second_monitor = &monitors[1];
                    let video_mode_opt = second_monitor.get_video_mode();
                    if let Some(video_mode) = video_mode_opt {
                        return glfw_entry.create_window(
                            video_mode.width,
                            video_mode.height,
                            title,
                            WindowMode::FullScreen(second_monitor),
                        );
                    }
                }
                glfw_entry.create_window(700, 700, title, WindowMode::Windowed)
            })
            .expect("failed to create window");

        let vulkan_instance = VulkanInstance::new_from_glfw(vulkan_entry, glfw_entry);

        let vulkan_surface = VulkanSurface::new_from_glfw_window(&vulkan_instance, &window);

        let vulkan_device =
            VulkanDevice::select_suitable_device_for_surface(&vulkan_instance, &vulkan_surface)
                .expect("failed to select suitable device for surface");

        let vulkan_swapchain =
            VulkanSwapchain::new_from_device_and_surface(&vulkan_device, &vulkan_surface);

        let vulkan_shader_1 = VulkanShaderModule::from_file(
            &vulkan_device,
            "shaders/spir-v/base.vert.spv",
            vk::ShaderStageFlags::VERTEX,
        );
        let vulkan_shader_2 = VulkanShaderModule::from_file(
            &vulkan_device,
            "shaders/spir-v/base.frag.spv",
            vk::ShaderStageFlags::FRAGMENT,
        );

        let vulkan_graphics_pipeline = VulkanGraphicsPipeline::new(
            &vulkan_device,
            Extent2D::default().width(1920).height(1080),
            Format::R8G8B8A8_SNORM,
            &[vulkan_shader_1, vulkan_shader_2],
        );

        Application {
            glfw_window: window,
            glfw_events: events,
        }
    }

    pub fn run(&self) {
        sleep(Duration::from_secs(60));
    }
}
