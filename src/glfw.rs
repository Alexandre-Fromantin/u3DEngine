use std::{
    ffi::{CStr, c_char},
    ops::{Deref, DerefMut},
    slice,
};

use glfw::Glfw;

pub struct GlfwEntry<'glfw_lifetime> {
    entry: Glfw,

    ///All Vulkan Instance Extension Names required by GLFW (in *const c_char)
    pub all_req_vk_inst_ext_names_c: &'glfw_lifetime [*const c_char],

    ///All Vulkan Instance Extension Names required by GLFW (in str)
    pub all_req_vk_inst_ext_names: Box<[&'glfw_lifetime str]>,
}

impl<'glfw_lifetime> GlfwEntry<'glfw_lifetime> {
    pub fn init() -> Self {
        let entry = glfw::init(glfw::fail_on_errors).expect("failed to init GLFW library");

        let mut extension_count = 0;
        let all_required_instance_extension_names_ptr =
            unsafe { glfw::ffi::glfwGetRequiredInstanceExtensions(&mut extension_count) };

        let all_req_vk_inst_ext_names_c: &[*const c_char] = unsafe {
            slice::from_raw_parts(
                all_required_instance_extension_names_ptr,
                extension_count as usize,
            )
        };

        let mut all_req_vk_inst_ext_names_uninit =
            Box::new_uninit_slice(all_req_vk_inst_ext_names_c.len());

        for (index, required_instance_extension_name_c_char) in
            all_req_vk_inst_ext_names_c.iter().cloned().enumerate()
        {
            all_req_vk_inst_ext_names_uninit[index].write(unsafe {
                str::from_utf8_unchecked(
                    //Vulkan extension names are in UTF-8 format
                    CStr::from_ptr(required_instance_extension_name_c_char).to_bytes(),
                )
            });
        }

        let all_req_vk_inst_ext_names = unsafe { all_req_vk_inst_ext_names_uninit.assume_init() };

        GlfwEntry {
            entry,
            all_req_vk_inst_ext_names_c,
            all_req_vk_inst_ext_names,
        }
    }
}

impl Deref for GlfwEntry<'_> {
    type Target = Glfw;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

impl DerefMut for GlfwEntry<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entry
    }
}
