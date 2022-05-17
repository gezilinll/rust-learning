package com.example.wgpu;

import android.view.Surface;

public class RustBridge {
    static  {
        System.loadLibrary("wgpu_on_app");
    }

    public native long createWgpuCanvas(Surface surface, int idx);

    public native void enterFrame(long canvas);

    public native void changeExample(long canvas, int idx);

    public native void dropWgpuCanvas(long canvas);
}
