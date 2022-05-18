use jni::sys::{jobject, JNIEnv};
use log::info;
use raw_window_handle::{AndroidNdkHandle, HasRawWindowHandle, RawWindowHandle};
use std::ffi::c_void;

pub struct AppState {
    pub native_window: NativeWindow,
    pub surface: wgpu::Surface,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
}

impl AppState {
    pub fn new(env: *mut JNIEnv, surface: jobject) -> Self {
        let window =
            unsafe { ndk_sys::ANativeWindow_fromSurface(env as *mut _, surface as *mut _) };
        let window = unsafe { NativeWindow::new(window) };
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let _surface = unsafe { instance.create_surface(&window) };
        //todo:???
        let (_adapter, _device, _queue) =
            pollster::block_on(request_device(&instance, wgpu::Backends::all(), &_surface));
        //todo:???
        let _config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: window.get_width(),
            height: window.get_height(),
            present_mode: wgpu::PresentMode::Fifo,
        };
        _surface.configure(&_device, &_config);

        Self {
            native_window: window,
            surface: _surface,
            config: _config,
            queue: _queue,
            device: _device,
        }
    }
}

async fn request_device(
    instance: &wgpu::Instance,
    backend: wgpu::Backends,
    surface: &wgpu::Surface,
) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    let adapter =
        wgpu::util::initialize_adapter_from_env_or_default(instance, backend, Some(surface))
            .await
            .expect("No suitable GPU adapters found on the system!");
    let adapter_info = adapter.get_info();
    info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

    let res = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: adapter.features(),
                limits: adapter.limits(),
            },
            None,
        )
        .await;
    match res {
        Err(err) => {
            panic!("request_device failed {:?}", err);
        }
        Ok(tuple) => (adapter, tuple.0, tuple.1),
    }
}

pub struct NativeWindow {
    a_native_window: *mut ndk_sys::ANativeWindow,
}

impl NativeWindow {
    unsafe fn new(window: *mut ndk_sys::ANativeWindow) -> Self {
        ndk_sys::ANativeWindow_acquire(window);
        Self {
            a_native_window: window,
        }
    }

    fn get_width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(self.a_native_window) as u32 }
    }

    fn get_height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(self.a_native_window) as u32 }
    }
}

unsafe impl HasRawWindowHandle for NativeWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = AndroidNdkHandle::empty();
        //todo:???
        handle.a_native_window = self.a_native_window as *mut _ as *mut c_void;
        RawWindowHandle::AndroidNdk(handle)
    }
}
