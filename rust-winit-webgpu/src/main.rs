use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder},
};

mod graphics {
    use winit::window::Window;

    pub struct State {
        surface: wgpu::Surface,
        adapter: wgpu::Adapter,
        device: wgpu::Device,
        queue: wgpu::Queue,
        swap_chain: wgpu::SwapChain,
        sc_descriptor: wgpu::SwapChainDescriptor,
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
            Self {
                surface,
                adapter,
                device,
                queue,
                sc_descriptor,
                swap_chain,
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
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
