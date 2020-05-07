use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder},
};

mod graphics {
    use winit::window::Window;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 3],
    }

    unsafe impl bytemuck::Pod for Vertex {}
    unsafe impl bytemuck::Zeroable for Vertex {}

    impl Vertex {
        fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
            use std::mem;
            wgpu::VertexBufferDescriptor {
                stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float3,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float3,
                    },
                ]
            }
        }
    }


    const VERTICES: &[Vertex] = &[
        Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    ];

    pub struct State {
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        sc_descriptor: wgpu::SwapChainDescriptor,
        swap_chain: wgpu::SwapChain,
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
        window_size: winit::dpi::PhysicalSize<u32>,
    }

    impl State {
        pub async fn new(window: &Window) ->Self {
            let surface =  wgpu::Surface::create(window);
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface) }, wgpu::BackendBit::PRIMARY).await.unwrap();
            let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor
                 { extensions: wgpu::Extensions { anisotropic_filtering: false, }, limits: Default::default(), }).await;
            let sc_descriptor = wgpu::SwapChainDescriptor{
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: window.inner_size().width,
                height: window.inner_size().height,
                present_mode: wgpu::PresentMode::Fifo,
            };
            let swap_chain = device.create_swap_chain(&surface, &sc_descriptor);

            let vs_spirv = glsl_to_spirv::compile(include_str!("shader.vert"), glsl_to_spirv::ShaderType::Vertex).unwrap();
            let fs_spirv = glsl_to_spirv::compile(include_str!("shader.frag"), glsl_to_spirv::ShaderType::Fragment).unwrap();
            let vs_data = wgpu::read_spirv(vs_spirv).unwrap();
            let fs_data = wgpu::read_spirv(fs_spirv).unwrap();
            let vs_module = device.create_shader_module(&vs_data);
            let fs_module = device.create_shader_module(&fs_data);
            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { bind_group_layouts: &[]});
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main"
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format: sc_descriptor.format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }
                ],
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[Vertex::desc()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

            let vertex_buffer = device.create_buffer_with_data(bytemuck::cast_slice(VERTICES), wgpu::BufferUsage::VERTEX);

            Self {
                surface,
                device,
                queue,
                sc_descriptor,
                swap_chain,
                render_pipeline,
                vertex_buffer,
                window_size: window.inner_size(),
            }
        }

        pub async fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
            self.window_size = size;
            self.sc_descriptor.width = size.width;
            self.sc_descriptor.height = size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_descriptor);
        }

        pub async fn render(&mut self) {
            let frame = self.swap_chain.get_next_texture().expect("failed to get next texture");
            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("renderer encoder"),
            });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[
                        wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 1.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }
                        }
                    ],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
                render_pass.draw(0..VERTICES.len() as u32, 0..1);
            }
            self.queue.submit(&[encoder.finish()]);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    let mut state = futures::executor::block_on(graphics::State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                futures::executor::block_on(state.render());
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(physical_size) => {
                    futures::executor::block_on(state.resize(*physical_size));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                    futures::executor::block_on(state.resize(**new_inner_size));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input,
                    ..
                } => {
                    match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    }
                }
                _ => {}
            }
            _ => {}
        }
    });
}
