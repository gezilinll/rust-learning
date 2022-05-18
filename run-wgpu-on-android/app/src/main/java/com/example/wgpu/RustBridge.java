package com.example.wgpu;

import android.view.Surface;

public class RustBridge {
    static  {
        System.loadLibrary("wgpu_on_app");
    }

    public native long startRenderTriangle(Surface surface);
}
