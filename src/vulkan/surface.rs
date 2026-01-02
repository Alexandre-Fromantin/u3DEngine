use ash::vk;
use glfw::PWindow;

use crate::vulkan::instance::VulkanInstance;

///Represent a Vulkan surface
pub struct VulkanSurface<'vulkan_instance> {
    ///A reference to the Vulkan instance which created this surface
    vulkan_instance: &'vulkan_instance VulkanInstance,

    ///The Vulkan surface
    surface: vk::SurfaceKHR,
}

impl<'vulkan_instance> VulkanSurface<'vulkan_instance> {
    pub fn new_from_glfw_window(
        vulkan_instance: &'vulkan_instance VulkanInstance,
        glfw_window: &PWindow,
    ) -> Self {
        let hwnd = glfw_window.get_win32_window() as isize;
        let surface_create_info = vk::Win32SurfaceCreateInfoKHR {
            hwnd,
            ..Default::default()
        };
        let surface = vulkan_instance
            .create_win32_surface(&surface_create_info)
            .expect("failed to create Vulkan surface(SurfaceKHR)");

        Self {
            vulkan_instance,
            surface,
        }
    }

    ///Return all available present modes supported by `&self` and `physical_device`
    pub fn is_a_supported_device_queue(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        self.vulkan_instance
            .get_physical_device_win32_presentation_support(physical_device, queue_family_index)
    }

    ///Return all available present modes supported by `&self` and `physical_device`
    pub fn get_available_present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::PresentModeKHR> {
        self.vulkan_instance
            .get_physical_device_surface_present_modes(physical_device, self.surface)
            .expect("failed to get available present modes for surface")
    }

    ///Return all available formats supported by `&self` and `physical_device`
    pub fn get_available_formats(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::SurfaceFormatKHR> {
        self.vulkan_instance
            .get_physical_device_surface_formats(physical_device, self.surface)
            .expect("failed to get available present formats for surface")
    }

    ///Return capabilities of `&self` and `physical_device`
    pub fn get_available_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::SurfaceCapabilitiesKHR {
        self.vulkan_instance
            .get_physical_device_surface_capabilities(physical_device, self.surface)
            .expect("failed to get available present capabilities for surface")
    }

    pub fn set_surface_in_swapchain_create_info_khr(
        &self,
        swapchain_create_info: &mut vk::SwapchainCreateInfoKHR,
    ) {
        swapchain_create_info.surface = self.surface
    }
}

impl<'a> Drop for VulkanSurface<'a> {
    fn drop(&mut self) {
        self.vulkan_instance.destroy_surface(self.surface);
    }
}
