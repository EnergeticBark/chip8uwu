use egui::load::SizedTexture;
use egui::{ClippedPrimitive, Context, TexturesDelta, ViewportId};
use egui_wgpu;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::chip8::State;
use crate::ui::gui::Gui;
use crate::ui::Screen;

pub struct Framework<'window> {
    surface: wgpu::Surface<'window>,
    device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    // Chip8 screen.
    pub screen: Screen,
    // State for egui.
    egui_state: egui_winit::State,
    screen_descriptor: egui_wgpu::ScreenDescriptor,
    renderer: egui_wgpu::Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI.
    gui: Gui,
}

impl<'window> Framework<'window> {
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        window: &'window Window,
        width: u32,
        height: u32,
    ) -> Framework<'window> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::util::power_preference_from_env().unwrap_or_default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&Default::default(), None)).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let max_texture_side = device.limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            ViewportId::ROOT,
            &event_loop,
            Some(window.scale_factor() as f32),
            Some(max_texture_side),
        );
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: window.inner_size().into(),
            pixels_per_point: window.scale_factor() as f32,
        };
        let mut renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1);
        let textures = TexturesDelta::default();

        let screen = Screen::new(&device, width, height);

        let egui_texture =
            renderer.register_native_texture(&device, &screen.view, wgpu::FilterMode::Nearest);
        let sized_texture = SizedTexture::new(egui_texture, [64.0, 32.0]);
        let gui = Gui::new(sized_texture);

        Self {
            surface,
            device,
            queue,
            config,
            screen,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    pub(crate) fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_window_event(window, event);
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    pub(crate) fn prepare(&mut self, window: &Window, chip8_state: &mut State) {
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_state.egui_ctx().run(raw_input, |egui_ctx| {
            self.gui.ui(egui_ctx, chip8_state);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, output.platform_output);
        self.paint_jobs = self
            .egui_state
            .egui_ctx()
            .tessellate(output.shapes, self.screen_descriptor.pixels_per_point);
    }

    pub(crate) fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
