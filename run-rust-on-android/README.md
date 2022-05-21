# 在Android 上集成 Rust

## IDE 选择

Android Studio

- CLion 无法辩解运行 Android 工程
- VSCode 、IDEA 提供的 Android 开发与调试能力不如 AS 丰富或简单粗暴

## 编译环境配置

基础配置

- Android Studio
- SDK Manager 与对应 API 工具等
- JDK 11.0 或更高
- NDK
- Rust

指定架构依赖安装

- rustup target add armv7-linux-androideabi  # for arm
- rustup target add i686-linux-android    # for x86
- rustup target add aarch64-linux-android   # for arm64
- rustup target add x86_64-linux-android   # for x86_64

## 项目集成

**插件配置**

* 工程根目录下 build.gradle 中配置

```groovy
buildscript {
    repositories {
        maven {
            url "https://plugins.gradle.org/m2/"
        }
    }
    dependencies {
        classpath 'org.mozilla.rust-android-gradle:plugin:0.9.3'
    }
}
或者
buildscript {
    //...
}

plugins {
    id "org.mozilla.rust-android-gradle.rust-android" version "0.9.3"
}
```

* 项目（如app）目录下 build.gradle 中配置

```groovy
android { ... }

apply plugin: 'org.mozilla.rust-android-gradle.rust-android'

cargo {
    module  = "src/main/rust"       // Or whatever directory contains your Cargo.toml
    libname = "rust"          // Or whatever matches Cargo.toml's [package] name.
    targets = ["arm", "x86"]  // See bellow for a longer list of options
}
```

* 接下来就是在你的 app/src/main/rust 目录下创建 Rust 项目和编码
* 为了有更佳的编码体验：Android Studio → Preferences → Plugins → Marketplace 搜索并安装 Rust ，安装重启 AS 后，打开 Cargo.toml 文件，点击右上角 Attach 即可

## 代码调试

*目前 AS 上添加 Rust 代码断点只能在进入断点模式后，通过 LLDB 控制台输入断点命令的方式触发，相比 C++ 比较不方便。*

**操作步骤**

* 在项目（如app）目录下的build.gradle 中禁止编译器优化符号表

```groovy
android {
    ...

    packagingOptions {
        doNotStrip '**/*.so'
    }

    ...
}
```

* 配置 Debug Type 为 Dual

  * ![image2022-5-13_17-42-52](https://tva1.sinaimg.cn/large/e6c9d24ely1h2fscx2i6hj207502k3yg.jpg)
  * ![image2022-5-13_17-47-7](https://tva1.sinaimg.cn/large/e6c9d24ely1h2fsd6bh3uj20q607hmxv.jpg)

* Debug 模式下运行，然后在下方的调试界面的 app 栏目下暂停程序，启动 LLDB 控制台

  * ![image2022-5-13_17-47-7](https://tva1.sinaimg.cn/large/e6c9d24ely1h2fsdfqfruj20q607hmxv.jpg)

* 在控制台输入想要断点的位置，语法详见：[Android调试利器之LLDB](https://blog.csdn.net/wangyiyungw/article/details/81069631) ，这里举例上述示范项目中的 lib.rs 的第 26 行：

  - ```
    b /Users/linbinghe/Projects/rust-learning/run-rust-on-android/app/src/main/rust/src/lib.rs:26
    ```

## JNI

**操作步骤**

* 于 Cargo.toml 中配置依赖项

```toml
...
[dependencies]
jni = "0.19.0"
jni_fn = "0.1"
...
```

* 在 JNI 代码文件中（如 src/main/rust/src/lib.rs） 中实现 JAVA/Kotlin 上映射的 native 接口
* JNI 层的语法逻辑与 C++ 版本基本一致，这里不再赘述，没了解的可以进一步阅读下：https://zhuanlan.zhihu.com/p/97691316

# 参考

[rust-android-gradle](https://github.com/mozilla/rust-android-gradle)

[Is there any way to debug rust lib on android application?](https://github.com/mozilla/rust-android-gradle/issues/22)

[clion-rust-android-debug-howto](https://github.com/icota/clion-rust-android-debug-howto)

[jni](https://docs.rs/jni/latest/jni/)

[jni_fn](https://crates.io/crates/jni_fn)