use crate::{app::Application, glfw::GlfwEntry, vulkan::entry::VulkanEntry};

mod app;
mod glfw;
mod vulkan;

fn main() {
    let mut glfw_entry = GlfwEntry::init();
    let vulkan_entry = VulkanEntry::init();

    let app = Application::new(&mut glfw_entry, &vulkan_entry, "u3DEngine(Vulkan)");

    app.run();
}
/*
    const SHADER_ENTRYPOINT: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };

    let vert_shader_stage_create_info = PipelineShaderStageCreateInfo::default()
        .stage(ShaderStageFlags::VERTEX)
        .module(vert_shader_module)
        .name(SHADER_ENTRYPOINT);
    let frag_shader_stage_create_info = PipelineShaderStageCreateInfo::default()
        .stage(ShaderStageFlags::FRAGMENT)
        .module(frag_shader_module)
        .name(SHADER_ENTRYPOINT);

    while !window.should_close() {
        glfw.poll_events();
    }

    unsafe {
        swapchain_device.destroy_swapchain(swapchain, None);
        surface_instance.destroy_surface(surface, None);
        vulkan_instance.destroy_instance(None);
    }

    /*unsafe {
        instance.destroy_instance(None);
    }*/
}*/
