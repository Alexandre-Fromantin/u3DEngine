use ash::khr::win32_surface;
use ash::vk;
use glfw::PWindow;

use crate::vulkan::entry::VulkanEntry;
use crate::vulkan::instance::VulkanInstance;

pub struct VulkanSurface {
    win32_surface_instance: win32_surface::Instance,
    surface: vk::SurfaceKHR,
}

impl VulkanSurface {
    pub fn new_from_glfw_window(
        vulkan_entry: &VulkanEntry,
        vulkan_instance: &VulkanInstance,
        glfw_window: &PWindow,
    ) -> Self {
        let win32_surface_instance = win32_surface::Instance::new(vulkan_entry, vulkan_instance);

        let hwnd = glfw_window.get_win32_window() as isize;
        let surface_create_info = vk::Win32SurfaceCreateInfoKHR {
            hwnd,
            ..Default::default()
        };
        let surface =
            unsafe { win32_surface_instance.create_win32_surface(&surface_create_info, None) }
                .expect("failed to create Vulkan surface(SurfaceKHR)");

        Self {
            win32_surface_instance,
            surface,
        }
    }
}
