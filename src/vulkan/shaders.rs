use core::panic;
use std::{fs::File, io::Read};

use ash::vk;

use crate::vulkan::device::VulkanDevice;

pub struct VulkanShaderModule<'vulkan_device> {
    vulkan_device: &'vulkan_device VulkanDevice,
    shader_module: vk::ShaderModule,
}

impl<'vulkan_device> VulkanShaderModule<'vulkan_device> {
    pub fn from_file(vulkan_device: &'vulkan_device VulkanDevice, file_path: &str) -> Self {
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
        }
    }
}

impl Drop for VulkanShaderModule<'_> {
    fn drop(&mut self) {
        self.vulkan_device.destroy_shader_module(self.shader_module)
    }
}
