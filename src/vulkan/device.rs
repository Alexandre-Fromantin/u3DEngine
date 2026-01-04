use std::{
    ffi::{CStr, c_char},
    mem::MaybeUninit,
};

use ash::{
    khr,
    prelude::VkResult,
    vk::{
        self, GraphicsPipelineCreateInfo, PipelineLayoutCreateInfo, QueueFlags,
        RenderPassCreateInfo,
    },
};

use crate::vulkan::{instance::VulkanInstance, surface::VulkanSurface};

//TODO: Create a feature to select suitable devices from parameters
/*pub struct RequestedDeviceSurfaceSpecs {
    request_formats: Vec<SurfaceFormatKHR>,
    request_presents_mode: Vec<PresentModeKHR>,
}

pub struct SuitableDeviceParam<T> {
    must_have: T,
}*/

///Represent the queues of a Logical Device, which are used for graphics and presentations supported for a Vulkan Surface
enum DeviceQueue {
    ///The graphics and representation are used in the same Logical Queue, because they have the same family queue
    UniqueQueue(vk::Queue),

    ///The graphics and representation each have a dedicated queue, because they have different family queues
    TwoQueue {
        queue_family_indices: [u32; 2],
        graphics: vk::Queue,
        presents: vk::Queue,
    },
}

///Represent a Vulkan device
pub struct VulkanDevice {
    ///A Vulkan Physical Device
    ///
    ///For example, a reference to an RTX 5090
    physical_device: vk::PhysicalDevice,

    ///A Vulkan Logical Device, which created by Vulkan and linked to ```physical_device```
    logical_device: ash::Device,

    ///The queues of ```logical_device```, which are used for graphics and presentations supported for a Vulkan Surface
    queue: DeviceQueue,

    ///The Swapchain Device linked to Vulkan instance and ```logical_device```
    ///
    ///For example, can be used to create a swapchain
    swapchain_device: khr::swapchain::Device,
}

const REQUIRED_EXTENSION_NAME_FOR_SURFACE: [&CStr; 1] = [vk::KHR_SWAPCHAIN_NAME];

impl VulkanDevice {
    /// Create a VulkanDevice containing a physical and a logical device, the queues necessary to `surface`, and a swapchain device for managing future swapchains created from this VulkanDevice
    ///
    /// It's created to be usable with `surface`
    ///
    /// # Arguments
    ///
    /// * `vulkan_instance` - a reference to a VulkanInstance
    /// * `surface` - the VulkanDevice is created to be usable with this VulkanSurface ref
    ///
    /// # Returns
    ///
    /// None if none physical device can be used for this `surface`
    pub fn select_suitable_device_for_surface(
        vulkan_instance: &VulkanInstance,
        surface: &VulkanSurface,
    ) -> Option<Self> {
        let all_available_physical_devices =
            unsafe { vulkan_instance.enumerate_physical_devices() }
                .expect("failed to enumerate available physical devices");

        for physical_device in all_available_physical_devices {
            if !is_physical_device_supported_extensions(
                vulkan_instance,
                physical_device,
                &REQUIRED_EXTENSION_NAME_FOR_SURFACE,
            ) {
                continue;
            }

            let graphics_presents_queue_family_id_opt =
                GraphicsAndPresentQueueFamilyId::get_from_device_and_surface(
                    vulkan_instance,
                    physical_device,
                    surface,
                );
            if graphics_presents_queue_family_id_opt.is_none() {
                continue;
            }
            let graphics_presents_queue_family_id = graphics_presents_queue_family_id_opt.unwrap();

            let mut queue_create_infos: [MaybeUninit<vk::DeviceQueueCreateInfo>; 2] =
                [MaybeUninit::uninit(); 2];

            let queue_priotities = 1.0f32;

            let nb_family_queue = if graphics_presents_queue_family_id.is_same_queue_family() {
                queue_create_infos[0].write(vk::DeviceQueueCreateInfo {
                    queue_family_index: graphics_presents_queue_family_id.graphics,
                    p_queue_priorities: &queue_priotities,
                    queue_count: 2,
                    ..Default::default()
                });

                1
            } else {
                queue_create_infos[0].write(vk::DeviceQueueCreateInfo {
                    queue_family_index: graphics_presents_queue_family_id.graphics,
                    p_queue_priorities: &queue_priotities,
                    queue_count: 1,
                    ..Default::default()
                });
                queue_create_infos[1].write(vk::DeviceQueueCreateInfo {
                    queue_family_index: graphics_presents_queue_family_id.presents,
                    p_queue_priorities: &queue_priotities,
                    queue_count: 1,
                    ..Default::default()
                });

                2
            };

            let device_create_info = vk::DeviceCreateInfo {
                p_queue_create_infos: queue_create_infos.as_ptr()
                    as *const vk::DeviceQueueCreateInfo,
                queue_create_info_count: nb_family_queue,
                enabled_extension_count: REQUIRED_EXTENSION_NAME_FOR_SURFACE.len() as u32,
                pp_enabled_extension_names: REQUIRED_EXTENSION_NAME_FOR_SURFACE.as_ptr()
                    as *const *const c_char,
                ..Default::default()
            };

            let logical_device = unsafe {
                vulkan_instance.create_device(physical_device, &device_create_info, None)
            }
            .expect("failed to create Vulkan logical device");

            let queue = unsafe {
                if graphics_presents_queue_family_id.is_same_queue_family() {
                    DeviceQueue::UniqueQueue(
                        logical_device
                            .get_device_queue(graphics_presents_queue_family_id.graphics, 0),
                    )
                } else {
                    DeviceQueue::TwoQueue {
                        queue_family_indices: [
                            graphics_presents_queue_family_id.graphics,
                            graphics_presents_queue_family_id.presents,
                        ],
                        graphics: logical_device
                            .get_device_queue(graphics_presents_queue_family_id.graphics, 0),
                        presents: logical_device
                            .get_device_queue(graphics_presents_queue_family_id.presents, 0),
                    }
                }
            };

            let swapchain_device = khr::swapchain::Device::new(vulkan_instance, &logical_device);

            return Some(Self {
                logical_device,
                physical_device,
                queue,
                swapchain_device,
            });
        }
        None
    }

    ///Return all available present modes supported by `&self` and `surface`
    pub fn get_available_present_modes_for_surface(
        &self,
        surface: &VulkanSurface,
    ) -> Vec<vk::PresentModeKHR> {
        surface.get_available_present_modes(self.physical_device)
    }

    ///Return all available formats supported by `&self` and `surface`
    pub fn get_available_formats_for_surface(
        &self,
        surface: &VulkanSurface,
    ) -> Vec<vk::SurfaceFormatKHR> {
        surface.get_available_formats(self.physical_device)
    }

    ///Return capabilities of `&self` and `surface`
    pub fn get_available_capabilities_for_surface(
        &self,
        surface: &VulkanSurface,
    ) -> vk::SurfaceCapabilitiesKHR {
        surface.get_available_capabilities(self.physical_device)
    }

    ///Create a SwapchainHKR from `&self` and `swapchain_create_info`
    ///
    ///The image_sharing_mode, queue_family_index_count and p_queue_family_indices fields of `swapchain_create_info` are updated from `&self` to create the Swapchain
    pub fn create_swapchain(
        &self,
        swapchain_create_info: &mut vk::SwapchainCreateInfoKHR,
    ) -> vk::SwapchainKHR {
        if let DeviceQueue::TwoQueue {
            queue_family_indices,
            graphics: _,
            presents: _,
        } = &self.queue
        {
            swapchain_create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            swapchain_create_info.queue_family_index_count = 2;
            swapchain_create_info.p_queue_family_indices = queue_family_indices.as_ptr();
        } else {
            swapchain_create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
        }

        unsafe {
            self.swapchain_device
                .create_swapchain(swapchain_create_info, None)
        }
        .expect("failed to create Vulkan SwapchainHKR")
    }

    ///Destroy `swapchain` with swapchain_device present in `&self`
    pub fn destroy_swapchain(&self, swapchain: vk::SwapchainKHR) {
        unsafe { self.swapchain_device.destroy_swapchain(swapchain, None) };
    }

    pub fn create_shader_module(
        &self,
        shader_module_create_info: &vk::ShaderModuleCreateInfo,
    ) -> VkResult<vk::ShaderModule> {
        unsafe {
            self.logical_device
                .create_shader_module(shader_module_create_info, None)
        }
    }

    pub fn destroy_shader_module(&self, shader_module: vk::ShaderModule) {
        unsafe {
            self.logical_device
                .destroy_shader_module(shader_module, None)
        }
    }

    pub fn create_pipeline_layout(
        &self,
        pipeline_layout_create_info: &PipelineLayoutCreateInfo,
    ) -> VkResult<vk::PipelineLayout> {
        unsafe {
            self.logical_device
                .create_pipeline_layout(pipeline_layout_create_info, None)
        }
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: vk::PipelineLayout) {
        unsafe {
            self.logical_device
                .destroy_pipeline_layout(pipeline_layout, None)
        }
    }

    pub fn create_render_pass(
        &self,
        render_pass_create_info: &RenderPassCreateInfo,
    ) -> VkResult<vk::RenderPass> {
        unsafe {
            self.logical_device
                .create_render_pass(render_pass_create_info, None)
        }
    }

    pub fn destroy_render_pass(&self, render_pass: vk::RenderPass) {
        unsafe { self.logical_device.destroy_render_pass(render_pass, None) }
    }

    pub fn create_graphics_pipeline(
        &self,
        graphics_pipeline_create_info: GraphicsPipelineCreateInfo,
    ) -> VkResult<vk::Pipeline> {
        let create_res = unsafe {
            self.logical_device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[graphics_pipeline_create_info],
                None,
            )
        };

        match create_res {
            Ok(pipeline_vec) => Ok(pipeline_vec[0]),
            Err(error) => Err(error.1),
        }
    }

    pub fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        unsafe { self.logical_device.destroy_pipeline(pipeline, None) }
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {
        unsafe { self.logical_device.destroy_device(None) };
    }
}

/// Return if the `physical_device` supported `all_required_extension_names`
///
/// # Arguments
///
/// * `vulkan_instance` - a reference to a VulkanInstance
/// * `physical_device` - a Vulkan physical devices
/// * `all_required_extension_names` - Slice of all extension names to check
fn is_physical_device_supported_extensions(
    vulkan_instance: &VulkanInstance,
    physical_device: vk::PhysicalDevice,
    all_required_extension_names: &[&CStr],
) -> bool {
    let all_device_extension_properties =
        unsafe { vulkan_instance.enumerate_device_extension_properties(physical_device) }
            .expect("failed to enumerate device extension properties");

    for required_extension_name in all_required_extension_names.iter().cloned() {
        let mut found = false;
        for device_extension_properties in &all_device_extension_properties {
            let device_ext_name =
                unsafe { CStr::from_ptr(device_extension_properties.extension_name.as_ptr()) };
            if device_ext_name == required_extension_name {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    true
}

struct GraphicsAndPresentQueueFamilyId {
    graphics: u32,
    presents: u32,
}

impl GraphicsAndPresentQueueFamilyId {
    fn get_from_device_and_surface(
        vulkan_instance: &VulkanInstance,
        physical_device: vk::PhysicalDevice,
        surface: &VulkanSurface,
    ) -> Option<Self> {
        let all_queue_family_properties =
            unsafe { vulkan_instance.get_physical_device_queue_family_properties(physical_device) };

        let mut graphics_queue_family_id_opt = None;
        let mut presents_queue_family_id_opt = None;
        for (queue_family_id, queue_family_properties) in
            all_queue_family_properties.iter().enumerate()
        {
            if graphics_queue_family_id_opt.is_none()
                && queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            {
                graphics_queue_family_id_opt = Some(queue_family_id as u32);
            }

            if presents_queue_family_id_opt.is_none()
                && surface.is_a_supported_device_queue(physical_device, queue_family_id as u32)
            {
                presents_queue_family_id_opt = Some(queue_family_id as u32);
            }

            if let Some(graphics_queue_family_id) = graphics_queue_family_id_opt
                && let Some(presents_queue_family_id) = presents_queue_family_id_opt
            {
                return Some(GraphicsAndPresentQueueFamilyId {
                    graphics: graphics_queue_family_id,
                    presents: presents_queue_family_id,
                });
            }
        }

        None
    }

    fn is_same_queue_family(&self) -> bool {
        self.graphics == self.presents
    }
}
