use android_logger::Config;
use jni::objects::JClass;
use jni::sys::{jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;
use log::{error, Level};

mod app_state; // 得配置，不然 triangle 里面也找不到
mod triangle;

#[no_mangle]
#[jni_fn("com.example.wgpu.RustBridge")]
pub unsafe fn startRenderTriangle(env: *mut JNIEnv, _: JClass, surface: jobject) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    error!("[LBH] Renderer Launching...");
    //todo:???
    triangle::start_render(env as *mut _, surface);
    error!("[LBH] Renderer Launched");
}
