use pollster::block_on;
use wgpu::{Instance, Adapter, Backends, Dx12Compiler, PowerPreference, Surface, Device, Queue, Limits, Features};
use winit::window::Window;

pub struct WgpuSettings(pub Backends, pub Dx12Compiler, pub PowerPreference, pub Features, pub Limits);

pub fn init_wgpu(window: &Window, settings: WgpuSettings) -> (Instance, Surface, Adapter, Device, Queue) {
    let instance = init_instance(settings.0, settings.1);
    let surface = unsafe { instance.create_surface(window) }.unwrap();
    let adapter = init_adapter(&instance, &surface, settings.2);
    let (device, queue) = init_device(&adapter, settings.3, settings.4);
    
    (instance, surface, adapter, device, queue)
}

pub fn init_instance(backends: Backends, dx12_shaders: Dx12Compiler) -> Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        dx12_shader_compiler: dx12_shaders,
    })
}

pub fn init_adapter(instance: &Instance, surface: &Surface, pref_gpu: PowerPreference) -> Adapter {
    block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: pref_gpu,
        compatible_surface: Some(surface),
        force_fallback_adapter: false,
    })).unwrap()
}

pub fn init_device(adapter: &Adapter, features: Features, limits: Limits) -> (Device, Queue) {
    block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features,
            limits,
        },
        None,
    )).unwrap()
}