use {
    anyhow::{Context, Result},
    winit::{
        event::{Event, WindowEvent,MouseScrollDelta,DeviceEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
};
use winit::dpi::LogicalPosition;
use std::time::Instant;

mod render;
mod math;
mod camera;

use crate::camera::Camera;
use crate::math::Vec4;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[pollster::main]
async fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window_size = winit::dpi::PhysicalSize::new(WIDTH, HEIGHT);
    let window = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("GPU Path Tracer".to_string())
        .build(&event_loop)?;
    window.set_cursor_grab(winit::window::CursorGrabMode::Locked);
    let (device, queue, surface) = connect_to_gpu(&window).await?;

    // TODO: initialize renderer
    let (device, queue, surface) = (device, queue, surface);
    let mut renderer = render::PathTracer::new(device, queue);
    let mut camera = Camera::look_at(
            Vec4::new(0., 0.75, 1.),
            Vec4::new(0., -0.5, -1.),
            Vec4::new(0., 1., 0.),
    ); 
    let mut last_frame_time = Instant::now();
    event_loop.run(|event, control_handle| {
        control_handle.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_handle.exit(),
                WindowEvent::RedrawRequested => {
                    // Wait for the next available frame buffer.
                
                    
                    let frame: wgpu::SurfaceTexture = surface
                        .get_current_texture()
                        .expect("failed to get current texture");

                    // TODO: draw frame
                    let render_target = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    renderer.render_frame(&render_target,&camera);
                    frame.present();
                    window.request_redraw();
                }
                    WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                    // TODO: handle keyboard input for camera translation
                    camera.translate(event);
                    renderer.reset_samples();
                    
                }

                _ => (),
            },
               Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseWheel { delta } => {
                    let delta = match delta {
                        MouseScrollDelta::PixelDelta(delta) => 0.001 * delta.y as f32,
                        MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                    };
                    camera.zoom(delta);
                    renderer.reset_samples();
                }
                DeviceEvent::MouseMotion { delta } => {
                    let (dx, dy) = delta;
                    let sensitivity = 0.002;
                    camera.rotate(-dx as f32 * sensitivity, dy as f32 * sensitivity);
                    renderer.reset_samples();

                   //s window.set_cursor_position(LogicalPosition::new(0.0, 0.0)).unwrap();
                },
                _ => (),
            },
            _ => (),
        }
    })?;
    Ok(())
}

async fn connect_to_gpu(
    window: &Window
) -> Result<(wgpu::Device, wgpu::Queue, wgpu::Surface)> {
    use wgpu::TextureFormat::{Bgra8Unorm, Rgba8Unorm};

    // Create an "instance" of wgpu. This is the entry-point to the API.
    let instance = wgpu::Instance::default();

    // Create a drawable "surface" that is associated with the window.
    let surface = instance.create_surface(window)?;

    // Request a GPU that is compatible with the surface. If the system has multiple GPUs then
    // pick the high performance one.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .context("failed to find a compatible adapter")?;

    // Connect to the GPU. "device" represents the connection to the GPU and allows us to create
    // resources like buffers, textures, and pipelines. "queue" represents the command queue that
    // we use to submit commands to the GPU.
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .context("failed to connect to the GPU")?;

    // Configure the texture memory backing the surface. Our renderer will draw to a surface
    // texture every frame.
    let caps = surface.get_capabilities(&adapter);
    let format = caps
        .formats
        .into_iter()
        .find(|it| matches!(it, Rgba8Unorm | Bgra8Unorm))
        .context("could not find preferred texture format (Rgba8Unorm or Bgra8Unorm)")?;
    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 3,
    };
    surface.configure(&device, &config);

    Ok((device, queue, surface))
}

