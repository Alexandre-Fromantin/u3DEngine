use core::panic;
use std::{ffi::CStr, fs::File, io::Read};

use ash::vk;

use crate::vulkan::device::VulkanDevice;

const SHADER_ENTRYPOINT: &CStr = c"main";

pub struct VulkanShaderModule<'vulkan_device> {
    vulkan_device: &'vulkan_device VulkanDevice,
    shader_module: vk::ShaderModule,
    shader_state: vk::ShaderStageFlags,
}

impl<'vulkan_device> VulkanShaderModule<'vulkan_device> {
    pub fn from_file(
        vulkan_device: &'vulkan_device VulkanDevice,
        file_path: &str,
        shader_state: vk::ShaderStageFlags,
    ) -> Self {
        let mut shader_file = File::open(file_path).unwrap();
        let mut shader_file_buf = Vec::new(); //TODO: Use a buffer pool
        shader_file.read_to_end(&mut shader_file_buf).unwrap();

        if shader_file_buf.len() % 4 != 0 {
            panic!("corrupted shader");
        }

        let shader_code: &[u32] = bytemuck::cast_slice(shader_file_buf.as_slice());

        let shader_module_create_info = vk::ShaderModuleCreateInfo::default().code(shader_code);
        let shader_module = vulkan_device
            .create_shader_module(&shader_module_create_info)
            .expect("failed to create a shader module");

        VulkanShaderModule {
            vulkan_device,
            shader_module,
            shader_state,
        }
    }

    pub fn set_pipeline_stage(
        &self,
        pipeline_shader_stage_create_info: &mut vk::PipelineShaderStageCreateInfo,
    ) {
        *pipeline_shader_stage_create_info = vk::PipelineShaderStageCreateInfo::default()
            .stage(self.shader_state)
            .module(self.shader_module)
            .name(SHADER_ENTRYPOINT)
    }
}

impl Drop for VulkanShaderModule<'_> {
    fn drop(&mut self) {
        self.vulkan_device.destroy_shader_module(self.shader_module)
    }
}
