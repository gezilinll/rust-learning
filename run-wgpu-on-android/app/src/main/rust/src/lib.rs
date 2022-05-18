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
    // [LBH NOTE]
    // _ underscore 介绍：
    // https://webcache.googleusercontent.com/search?q=cache:gRJO-1HIYnYJ:https://runrust.miraheze.org/wiki/Underscore+&cd=1&hl=zh-CN&ct=clnk&gl=us
    // 1、通配符
    // 2、部分场景下将影响变量生命周期
    // 3、规避 unused 的警告
    // 4、忽略类型声明
    // 5、匿名的声明周期声明，代码的间接性
    // 6、常量定义，不过好像没啥用处
    triangle::start_render(env as *mut _, surface);
    error!("[LBH] Renderer Launched");
}
