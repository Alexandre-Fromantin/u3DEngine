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
