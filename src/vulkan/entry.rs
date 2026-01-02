use std::{ffi::CStr, ops::Deref};

///Represent the Vulkan library entry point.
pub struct VulkanEntry {
    ///The Ash entry point.
    entry: ash::Entry,

    ///All available extension names of the Vulkan library.
    all_available_extension_names: Box<[Box<str>]>,

    ///All available layer names of the Vulkan library.
    all_available_layer_names: Box<[Box<str>]>,
}

impl VulkanEntry {
    pub fn init() -> Self {
        let entry = ash::Entry::linked();

        let all_available_extension_names = load_all_available_extension_names(&entry);
        let all_available_layer_names = load_all_available_layer_names(&entry);

        VulkanEntry {
            entry,
            all_available_extension_names,
            all_available_layer_names,
        }
    }

    pub fn is_extensions_supported(&self, extension_names: &[&str]) -> bool {
        for extension_name in extension_names.iter().cloned() {
            let mut found = false;
            for available_extension in self.all_available_extension_names.iter() {
                if available_extension.as_ref() == extension_name {
                    found = true;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }

    pub fn is_layers_supported(&self, layer_names: &[&str]) -> bool {
        for layer_name in layer_names.iter().cloned() {
            let mut found = false;
            for available_layer in self.all_available_layer_names.iter() {
                if available_layer.as_ref() == layer_name {
                    found = true;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}

impl Deref for VulkanEntry {
    type Target = ash::Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

fn load_all_available_extension_names(entry: &ash::Entry) -> Box<[Box<str>]> {
    let all_available_extension_properties =
        unsafe { entry.enumerate_instance_extension_properties(None) }
            .expect("failed to get available Vulkan instance extension");

    let mut all_available_extension_names =
        Vec::with_capacity(all_available_extension_properties.len());

    for extension_properties in all_available_extension_properties {
        let extension_name =
            unsafe { CStr::from_ptr(extension_properties.extension_name.as_ptr()) }
                .to_string_lossy()
                .into_owned()
                .into_boxed_str();
        all_available_extension_names.push(extension_name);
    }

    all_available_extension_names.into_boxed_slice()
}

fn load_all_available_layer_names(entry: &ash::Entry) -> Box<[Box<str>]> {
    let all_available_layer_properties_vec = unsafe { entry.enumerate_instance_layer_properties() }
        .expect("failed to get available Vulkan layer extension");

    let mut all_available_layer_names_vec =
        Vec::with_capacity(all_available_layer_properties_vec.len());

    for layer_properties in all_available_layer_properties_vec {
        let layer_name = unsafe { CStr::from_ptr(layer_properties.layer_name.as_ptr()) }
            .to_string_lossy()
            .into_owned()
            .into_boxed_str();
        all_available_layer_names_vec.push(layer_name);
    }

    all_available_layer_names_vec.into_boxed_slice()
}
