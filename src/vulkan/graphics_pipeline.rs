use std::mem::MaybeUninit;

use ash::vk::{
    self, AttachmentLoadOp, AttachmentStoreOp, ColorComponentFlags, CullModeFlags, Extent2D,
    Format, FrontFace, ImageLayout, Offset2D, PipelineBindPoint, PipelineLayout,
    PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, SampleCountFlags,
};

use crate::vulkan::{device::VulkanDevice, shaders::VulkanShaderModule};

pub struct VulkanGraphicsPipeline<'vulkan_device> {
    vulkan_device: &'vulkan_device VulkanDevice,
    pipeline: vk::Pipeline,
    pipeline_layout: PipelineLayout,
    render_pass: vk::RenderPass,
}

impl<'vulkan_device> VulkanGraphicsPipeline<'vulkan_device> {
    pub fn new(
        vulkan_device: &'vulkan_device VulkanDevice,
        swapchain_extend: Extent2D,
        swapchain_format: Format,
        all_shader_modules: &[VulkanShaderModule],
    ) -> Self {
        let mut all_shader_stages_maybe_uninit: Box<
            [MaybeUninit<vk::PipelineShaderStageCreateInfo>],
        > = vec![MaybeUninit::uninit(); all_shader_modules.len()].into_boxed_slice();

        for (index, shader_module) in all_shader_modules.iter().enumerate() {
            let pipeline_shader_stage_create_info_maybeuninit = MaybeUninit::uninit();
            let mut pipeline_shader_stage_create_info =
                unsafe { pipeline_shader_stage_create_info_maybeuninit.assume_init() };

            shader_module.set_pipeline_stage(&mut pipeline_shader_stage_create_info);
            all_shader_stages_maybe_uninit[index].write(pipeline_shader_stage_create_info);
        }

        let all_shader_stages = unsafe { all_shader_stages_maybe_uninit.assume_init() };

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let pipeline_dynamic_state_create_info =
            vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

        let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::default();

        let input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport::default()
            .x(0f32)
            .y(0f32)
            .width(swapchain_extend.width as f32)
            .height(swapchain_extend.height as f32)
            .min_depth(0f32)
            .max_depth(1f32);

        let scissor = vk::Rect2D::default()
            .offset(Offset2D::default().x(0).y(0))
            .extent(swapchain_extend);

        let viewport_state_create_info = PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);

        let rasterization_state_create_info = vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(PolygonMode::FILL)
            .line_width(1f32)
            .cull_mode(CullModeFlags::BACK)
            .front_face(FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling_state_create_info = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(SampleCountFlags::TYPE_1);

        let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(ColorComponentFlags::RGBA)
            .blend_enable(false);

        let color_blend_attchement = [color_blend_attachment_state];
        let color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&color_blend_attchement);

        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = vulkan_device
            .create_pipeline_layout(&pipeline_layout_create_info)
            .expect("failed to create pipeline layout");

        let attachement_description = vk::AttachmentDescription::default()
            .format(swapchain_format)
            .samples(SampleCountFlags::TYPE_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);

        let color_attachement = vk::AttachmentReference::default()
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .attachment(0);

        let color_attachements = [color_attachement];
        let subpass_description = vk::SubpassDescription::default()
            .color_attachments(&color_attachements)
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS);

        let attachments = [attachement_description];
        let subpasses = [subpass_description];
        let render_pass_create_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses);

        let render_pass = vulkan_device
            .create_render_pass(&render_pass_create_info)
            .expect("failed to create render pass");

        let graphics_pipeline_create_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&all_shader_stages)
            .vertex_input_state(&vertex_input_state_create_info)
            .input_assembly_state(&input_assembly_state_create_info)
            .viewport_state(&viewport_state_create_info)
            .rasterization_state(&rasterization_state_create_info)
            .multisample_state(&multisampling_state_create_info)
            .color_blend_state(&color_blend_state_create_info)
            .dynamic_state(&pipeline_dynamic_state_create_info)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);

        let pipeline = vulkan_device
            .create_graphics_pipeline(graphics_pipeline_create_info)
            .expect("failed to create graphics pipeline");

        Self {
            vulkan_device,
            pipeline,
            pipeline_layout,
            render_pass,
        }
    }
}

impl Drop for VulkanGraphicsPipeline<'_> {
    fn drop(&mut self) {
        self.vulkan_device.destroy_pipeline(self.pipeline);
        self.vulkan_device
            .destroy_pipeline_layout(self.pipeline_layout);
        self.vulkan_device.destroy_render_pass(self.render_pass);
    }
}
