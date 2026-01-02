use std::{cmp::max, marker::PhantomData};

use ash::vk::{self, CompositeAlphaFlagsKHR, ImageUsageFlags, PresentModeKHR, SwapchainKHR};

use crate::vulkan::{device::VulkanDevice, surface::VulkanSurface};

const SUITABLE_PRESENT_MODES: vk::PresentModeKHR = vk::PresentModeKHR::MAILBOX;
const SUITABLE_SURFACE_FORMAT: vk::SurfaceFormatKHR = vk::SurfaceFormatKHR {
    format: vk::Format::B8G8R8A8_SRGB,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
};

pub struct VulkanSwapchain<'vulkan_instance, 'vulkan_device, 'vulkan_surface> {
    swapchain: SwapchainKHR,
    vulkan_device: &'vulkan_device VulkanDevice,
    phantom_data_surface: PhantomData<&'vulkan_surface VulkanSurface<'vulkan_instance>>,
}

impl<'vulkan_instance, 'vulkan_device, 'vulkan_surface>
    VulkanSwapchain<'vulkan_instance, 'vulkan_device, 'vulkan_surface>
{
    pub fn new_from_device_and_surface(
        vulkan_device: &'vulkan_device VulkanDevice,
        vulkan_surface: &'vulkan_surface VulkanSurface,
    ) -> Self {
        let available_surface_and_device_capabilities =
            vulkan_device.get_available_capabilities_for_surface(vulkan_surface);

        let available_surface_and_device_formats =
            vulkan_device.get_available_formats_for_surface(vulkan_surface);

        let available_surface_and_device_present_modes =
            vulkan_device.get_available_present_modes_for_surface(vulkan_surface);

        let mut swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .min_image_count(max(
                available_surface_and_device_capabilities.min_image_count,
                2,
            ))
            .image_extent(available_surface_and_device_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(available_surface_and_device_capabilities.current_transform)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .clipped(true);

        swapchain_create_info.present_mode =
            if available_surface_and_device_present_modes.contains(&SUITABLE_PRESENT_MODES) {
                SUITABLE_PRESENT_MODES
            } else {
                PresentModeKHR::FIFO //guaranteed to be available
            };

        let selected_format =
            if available_surface_and_device_formats.contains(&SUITABLE_SURFACE_FORMAT) {
                SUITABLE_SURFACE_FORMAT
            } else {
                available_surface_and_device_formats[0] //guaranteed to be available
            };
        swapchain_create_info.image_format = selected_format.format;
        swapchain_create_info.image_color_space = selected_format.color_space;

        vulkan_surface.set_surface_in_swapchain_create_info_khr(&mut swapchain_create_info);

        let swapchain = vulkan_device.create_swapchain(&mut swapchain_create_info);

        Self {
            swapchain,
            vulkan_device,
            phantom_data_surface: PhantomData,
        }
    }
}

impl Drop for VulkanSwapchain<'_, '_, '_> {
    fn drop(&mut self) {
        self.vulkan_device.destroy_swapchain(self.swapchain);
    }
}
