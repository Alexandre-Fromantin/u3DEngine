use ash::khr::win32_surface;
use ash::vk::{self, PhysicalDevice};
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

    pub fn is_a_supported_device_queue(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            self.win32_surface_instance
                .get_physical_device_win32_presentation_support(physical_device, queue_family_index)
        }
    }
}
