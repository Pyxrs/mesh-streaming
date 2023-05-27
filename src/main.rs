use flexi_logger::Logger;
use log::*;

use wgpu::{Surface, PowerPreference, Backends, Dx12Compiler, SurfaceConfiguration, TextureUsages, Device, Queue, SurfaceError, Features, Limits};
use wgpu_util::{WgpuSettings, init_wgpu};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod octree;
mod static_octree;
mod render;
mod wgpu_util;

struct State {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State {
    fn new(window: Window, settings: WgpuSettings) -> Self {
        let size = window.inner_size();
        let (_instance, surface, adapter, device, queue) = init_wgpu(&window, settings);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {}
}

fn main() {
    init_logger(Level::Info).unwrap();

    let (event_loop, mut state) = init_window(|w| w);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => state.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => state.resize(state.size),
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => state.window().request_redraw(),
            _ => {}
        }
    });
}

fn init_logger(level: Level) -> anyhow::Result<()> {
    Logger::try_with_env_or_str(level.as_str())?.start()?;
    anyhow::Ok(())
}

fn init_window<W>(window_builder: W) -> (EventLoop<()>, State) where W: FnOnce(WindowBuilder) -> WindowBuilder {
    let event_loop = EventLoop::new();
    let window: Window = window_builder(WindowBuilder::new()).build(&event_loop).unwrap();
    
    let settings = WgpuSettings(
        Backends::all(),
        Dx12Compiler::Fxc,
        PowerPreference::HighPerformance,
        Features::empty(),
        Limits::default()
    );

    (event_loop, State::new(window, settings))
}