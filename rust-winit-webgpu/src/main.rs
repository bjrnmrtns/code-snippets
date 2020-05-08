use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder},
};

mod graphics {
    use winit::window::Window;
    use image::GenericImageView;
    use cgmath::SquareMatrix;

    pub struct Camera {
        eye: cgmath::Point3<f32>,
        target: cgmath::Point3<f32>,
        up: cgmath::Vector3<f32>,
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    }

    impl Camera {
        fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
            // 1.
            let view = cgmath::Matrix4::look_at(self.eye, self.target, self.up);
            // 2.
            let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

            // 3.
            return OPENGL_TO_WGPU_MATRIX * proj * view;
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub struct Texture {
        pub texture: wgpu::Texture,
        pub view: wgpu::TextureView,
        pub sampler: wgpu::Sampler,
    }

    impl Texture {
        pub fn new(device: &wgpu::Device, bytes: &[u8]) -> (Self, wgpu::CommandBuffer) {
            let diffuse_image = image::load_from_memory(bytes).unwrap();
            let diffuse_rgba = diffuse_image.as_rgba8().unwrap();
            let size = wgpu::Extent3d {
                width: diffuse_image.dimensions().0,
                height: diffuse_image.dimensions().1,
                depth: 1,
            };
            let diffuse_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size,
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            });
            let buffer = device.create_buffer_with_data(diffuse_rgba.as_ref(), wgpu::BufferUsage::COPY_SRC);
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
                label: Some("texture_buffer_copy_encoder"),
            });
            encoder.copy_buffer_to_texture(wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * diffuse_image.dimensions().0,
                rows_per_image: diffuse_image.dimensions().1,
            },
                                           wgpu::TextureCopyView {
                                               texture: &diffuse_texture,
                                               mip_level: 0,
                                               array_layer: 0,
                                               origin: wgpu::Origin3d::ZERO,
                                           },
                                           size,
            );

            let cmd_buffer = encoder.finish();

            let diffuse_texture_view = diffuse_texture.create_default_view();
            let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: wgpu::CompareFunction::Always,
            });

            (Self { texture: diffuse_texture, view: diffuse_texture_view, sampler: diffuse_sampler }, cmd_buffer)
        }
        const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.

        pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor, label: &str) -> Self {
            let size = wgpu::Extent3d { // 2.
                width: sc_desc.width,
                height: sc_desc.height,
                depth: 1,
            };
            let desc = wgpu::TextureDescriptor {
                label: Some(label),
                size,
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: Self::DEPTH_FORMAT,
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                    | wgpu::TextureUsage::SAMPLED
                    | wgpu::TextureUsage::COPY_SRC,
            };
            let texture = device.create_texture(&desc);

            let view = texture.create_default_view();
            let sampler = device.create_sampler(&wgpu::SamplerDescriptor { // 4.
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: wgpu::CompareFunction::LessEqual, // 5.
            });

            Self { texture, view, sampler }
        }
    }


    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    struct Vertex {
        position: [f32; 3],
        tex_coords: [f32; 2],
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
                        format: wgpu::VertexFormat::Float2,
                    },
                ]
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Uniforms {
        view: cgmath::Matrix4<f32>,
    }

    unsafe impl bytemuck::Pod for Uniforms {}
    unsafe impl bytemuck::Zeroable for Uniforms {}

    impl Uniforms {
        pub fn new() -> Self {
            Self {
                view: cgmath::Matrix4::identity(),
            }
        }
        pub fn update_view(&mut self, camera: &Camera) {
            self.view = camera.build_view_projection_matrix();
        }
    }

    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958646], }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.050602943], }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.15267089], }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
    ];

    const INDICES: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Instance {
        model: cgmath::Matrix4<f32>,
    }

    unsafe impl bytemuck::Pod for Instance {}
    unsafe impl bytemuck::Zeroable for Instance {}

    pub struct State {
        surface: wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        sc_descriptor: wgpu::SwapChainDescriptor,
        swap_chain: wgpu::SwapChain,
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        texture: Texture,
        diffuse_bind_group: wgpu::BindGroup,
        camera: Camera,
        instance_buffer: wgpu::Buffer,
        uniforms: Uniforms,
        uniform_buffer: wgpu::Buffer,
        uniform_bind_group: wgpu::BindGroup,
        depth_texture: Texture,
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

            let vertex_buffer = device.create_buffer_with_data(bytemuck::cast_slice(VERTICES), wgpu::BufferUsage::VERTEX);
            let index_buffer = device.create_buffer_with_data(bytemuck::cast_slice(INDICES), wgpu::BufferUsage::INDEX);

            let (texture, cmd_buffer) = Texture::new(&device, include_bytes!("happy-tree.png"));

            queue.submit(&[cmd_buffer]);
            let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        }
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                        },
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

            let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
                layout: &texture_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            });

            let camera = Camera {
                eye: (0.0, 1.0, 2.0).into(),
                target: (0.0, 0.0, 0.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: sc_descriptor.width as f32 / sc_descriptor.height as f32,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            };

            let mut uniforms = Uniforms::new();
            uniforms.update_view(&camera);

            let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]),
                                                                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

            let instances = [Instance { model: cgmath::Matrix4::identity(), }, Instance { model: cgmath::Matrix4::from_translation(cgmath::Vector3 { x: 0.2 as f32, y: 0.2 as f32, z: -0.5 as f32}), }];
            let instances_buffer_size = instances.len() * std::mem::size_of::<cgmath::Matrix4<f32>>();
            let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&instances), wgpu::BufferUsage::STORAGE_READ);

            let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: true,
                    },
            },
            ],
            label: Some("uniform_bind_group_layout"),
            });

            let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buffer,
                            // FYI: you can share a single buffer between bindings.
                            range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                        }
                    },
                    wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &instance_buffer,
                        range: 0..instances_buffer_size as wgpu::BufferAddress,
                    }
            },
            ],
            label: Some("uniform_bind_group"),
            });

            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
            });

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
                depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                    format: Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_read_mask: 0,
                    stencil_write_mask: 0,
                }),
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[Vertex::desc()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });
            let depth_texture = Texture::create_depth_texture(&device, &sc_descriptor, "depth_texture");
            Self {
                surface,
                device,
                queue,
                sc_descriptor,
                swap_chain,
                render_pipeline,
                vertex_buffer,
                index_buffer,
                texture,
                diffuse_bind_group,
                camera,
                instance_buffer,
                uniforms,
                uniform_buffer,
                uniform_bind_group,
                depth_texture,
                window_size: window.inner_size(),
            }
        }

        pub async fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
            self.window_size = size;
            self.sc_descriptor.width = size.width;
            self.sc_descriptor.height = size.height;
            self.depth_texture = Texture::create_depth_texture(&self.device, &self.sc_descriptor, "depth_texture");
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
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                        attachment: &self.depth_texture.view,
                        depth_load_op: wgpu::LoadOp::Clear,
                        depth_store_op: wgpu::StoreOp::Store,
                        clear_depth: 1.0,
                        stencil_load_op: wgpu::LoadOp::Clear,
                        stencil_store_op: wgpu::StoreOp::Store,
                        clear_stencil: 0,
                    }),
                });
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
                render_pass.set_index_buffer(&self.index_buffer, 0 ,0);
                render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
                render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
//                render_pass.draw(0..VERTICES.len() as u32, 0..1);
                render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..2);
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
