use ash::vk;
use glfw::PWindow;

use crate::vulkan::instance::VulkanInstance;

pub struct VulkanSurface {
    surface: vk::SurfaceKHR,
}

impl VulkanSurface {
    pub fn new_from_glfw_window(vulkan_instance: &VulkanInstance, glfw_window: &PWindow) -> Self {
        let hwnd = glfw_window.get_win32_window() as isize;
        let surface_create_info = vk::Win32SurfaceCreateInfoKHR {
            hwnd,
            ..Default::default()
        };
        let surface = unsafe {
            vulkan_instance
                .win32_surface_instance
                .create_win32_surface(&surface_create_info, None)
        }
        .expect("failed to create Vulkan surface(SurfaceKHR)");

        Self { surface }
    }

    pub fn is_a_supported_device_queue(
        &self,
        vulkan_instance: &VulkanInstance,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            vulkan_instance
                .win32_surface_instance
                .get_physical_device_win32_presentation_support(physical_device, queue_family_index)
        }
    }

    pub fn get_available_present_modes(
        &self,
        vulkan_instance: &VulkanInstance,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::PresentModeKHR> {
        unsafe {
            vulkan_instance
                .surface_instance
                .get_physical_device_surface_present_modes(physical_device, self.surface)
        }
        .expect("failed to get available present modes for surface")
    }

    pub fn get_available_formats(
        &self,
        vulkan_instance: &VulkanInstance,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::SurfaceFormatKHR> {
        unsafe {
            vulkan_instance
                .surface_instance
                .get_physical_device_surface_formats(physical_device, self.surface)
        }
        .expect("failed to get available present formats for surface")
    }

    pub fn get_available_capabilities(
        &self,
        vulkan_instance: &VulkanInstance,
        physical_device: vk::PhysicalDevice,
    ) -> vk::SurfaceCapabilitiesKHR {
        unsafe {
            vulkan_instance
                .surface_instance
                .get_physical_device_surface_capabilities(physical_device, self.surface)
        }
        .expect("failed to get available present capabilities for surface")
    }
}
