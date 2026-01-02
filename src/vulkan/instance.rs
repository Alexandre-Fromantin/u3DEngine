use std::{
    ffi::{CStr, c_char},
    ops::Deref,
};

use ash::{khr, prelude::VkResult, vk};

use crate::{glfw::GlfwEntry, vulkan::entry::VulkanEntry};

pub struct VulkanInstance {
    instance: ash::Instance,

    surface_instance: khr::surface::Instance,
    win32_surface_instance: khr::win32_surface::Instance,
}

static VALIDATION_LAYER_NAME_C: &CStr = c"VK_LAYER_KHRONOS_validation";
static VALIDATION_LAYER_NAME: &str = match VALIDATION_LAYER_NAME_C.to_str() {
    Ok(v) => v,
    Err(_) => panic!("VALIDATION_LAYER_NAME_C.to_str() failed"),
};

const VULKAN_INSTANCE_LAYER_NAMES_C: [*const c_char; 1] =
    [VALIDATION_LAYER_NAME_C.as_ptr() as *const c_char];

const VULKAN_INSTANCE_LAYER_NAMES: [&str; 1] = [VALIDATION_LAYER_NAME];

impl VulkanInstance {
    pub fn new_from_glfw(vulkan_entry: &VulkanEntry, glfw_entry: &GlfwEntry) -> Self {
        let app_info = vk::ApplicationInfo::default();

        if !vulkan_entry.is_extensions_supported(&glfw_entry.all_req_vk_inst_ext_names) {
            panic!("one or more requested extensions aren't supported by Vulkan")
        }

        if !vulkan_entry.is_layers_supported(&VULKAN_INSTANCE_LAYER_NAMES) {
            panic!("one or more requested layers aren't supported by Vulkan")
        }

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&VULKAN_INSTANCE_LAYER_NAMES_C)
            .enabled_extension_names(glfw_entry.all_req_vk_inst_ext_names_c);

        let instance = unsafe { vulkan_entry.create_instance(&instance_create_info, None) }
            .expect("failed to create Vulkan instance");

        let surface_instance = khr::surface::Instance::new(vulkan_entry, &instance);
        let win32_surface_instance = khr::win32_surface::Instance::new(vulkan_entry, &instance);

        Self {
            instance,
            surface_instance,
            win32_surface_instance,
        }
    }

    pub fn create_win32_surface(
        &self,
        surface_create_info: &vk::Win32SurfaceCreateInfoKHR,
    ) -> VkResult<vk::SurfaceKHR> {
        unsafe {
            self.win32_surface_instance
                .create_win32_surface(surface_create_info, None)
        }
    }

    pub fn get_physical_device_win32_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> bool {
        unsafe {
            self.win32_surface_instance
                .get_physical_device_win32_presentation_support(physical_device, queue_family_index)
        }
    }

    pub fn get_physical_device_surface_present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            self.surface_instance
                .get_physical_device_surface_present_modes(physical_device, surface)
        }
    }

    pub fn get_physical_device_surface_formats(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            self.surface_instance
                .get_physical_device_surface_formats(physical_device, surface)
        }
    }

    pub fn get_physical_device_surface_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            self.surface_instance
                .get_physical_device_surface_capabilities(physical_device, surface)
        }
    }

    pub fn destroy_surface(&self, surface: vk::SurfaceKHR) {
        unsafe { self.surface_instance.destroy_surface(surface, None) }
    }
}

impl Drop for VulkanInstance {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}

impl Deref for VulkanInstance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}
