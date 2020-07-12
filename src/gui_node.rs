use crate::color::Color;
use crate::draw_command::DrawCommand;
use crate::gui_attributes::*;
use crate::pipeline::Pipeline;
use crate::resource_cache::ResourceCache;
use crate::texture::Texture;

pub struct GuiNode {
	pub form_factor: GuiNodeUniform,
	pub pipeline_name: String,
}

impl GuiNode {
	pub fn new(width: u32, height: u32, color: Color) -> Self {
		Self {
			form_factor: GuiNodeUniform::new(width, height, color),
			pipeline_name: String::from("gui_rect"),
		}
	}

	pub fn build_draw_commands_recursive(
		node: &rctree::Node<GuiNode>,
		device: &wgpu::Device,
		queue: &mut wgpu::Queue,
		pipeline_cache: &ResourceCache<Pipeline>,
		texture_cache: &mut ResourceCache<Texture>,
	) -> Vec<DrawCommand> {
		let mut draw_commands: Vec<DrawCommand> = Vec::new();

		for mut subnode in node.descendants() {
			let mut subnode_data = subnode.borrow_mut();
			let pipeline = pipeline_cache.get(&subnode_data.pipeline_name[..]).unwrap();
			let command = subnode_data.build_draw_command(device, queue, pipeline, texture_cache);
			draw_commands.push(command);
		}

		draw_commands
	}

	pub fn build_draw_command(&mut self, device: &wgpu::Device, queue: &mut wgpu::Queue, pipeline: &Pipeline, texture_cache: &mut ResourceCache<Texture>) -> DrawCommand {
		const VERTICES: &[[f32; 2]] = &[[-0.5, 0.5], [0.5, 0.5], [0.5, 1.0], [-0.5, 1.0]];
		const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

		let bind_groups = self.build_bind_groups(device, queue, pipeline, texture_cache);

		// Create a draw command with the vertex data then push it to the GPU command queue
		DrawCommand::new(device, self.pipeline_name.clone(), bind_groups, VERTICES, INDICES)
	}

	pub fn build_bind_groups(&mut self, device: &wgpu::Device, queue: &mut wgpu::Queue, pipeline: &Pipeline, texture_cache: &mut ResourceCache<Texture>) -> Vec<wgpu::BindGroup> {
		// Load the cached texture
		let texture = Texture::cached_load(device, queue, "textures/grid.png", texture_cache);

		// Build a staging buffer from the uniform resource data
		let binding_staging_buffer = Pipeline::build_binding_staging_buffer(device, &self.form_factor);

		// Construct the bind group for this GUI node
		let bind_group = Pipeline::build_bind_group(
			device,
			&pipeline.bind_group_layout,
			vec![
				Pipeline::build_binding_resource(&binding_staging_buffer),
				wgpu::BindingResource::TextureView(&texture.texture_view),
				wgpu::BindingResource::Sampler(&texture.sampler),
			],
		);

		vec![bind_group]
	}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GuiNodeUniform {
	pub dimensions: Dimensions<u32>,
	pub corners_radius: Corners<f32>,
	pub sides_inset: Sides<f32>,
	pub border_thickness: f32,
	pub border_color: Color,
	pub fill_color: Color,
}

impl GuiNodeUniform {
	pub fn new(width: u32, height: u32, color: Color) -> Self {
		GuiNodeUniform {
			dimensions: Dimensions::<u32>::new(width, height),
			corners_radius: Corners::<f32>::new(0.0, 0.0, 0.0, 0.0),
			sides_inset: Sides::<f32>::new(0.0, 0.0, 0.0, 0.0),
			border_thickness: 0.0,
			border_color: Color::TRANSPARENT,
			fill_color: color,
		}
	}
}

unsafe impl bytemuck::Zeroable for GuiNodeUniform {}
unsafe impl bytemuck::Pod for GuiNodeUniform {}
