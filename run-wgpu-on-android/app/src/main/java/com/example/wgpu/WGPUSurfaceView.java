package com.example.wgpu;

import android.content.Context;
import android.util.AttributeSet;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

import androidx.annotation.NonNull;

public class WGPUSurfaceView extends SurfaceView implements SurfaceHolder.Callback {
    private RustBridge mRustBridge = new RustBridge();
    private long mNativeCanvas = Long.MAX_VALUE;
    private int mExampleIndex = 0;

    public WGPUSurfaceView(Context context) {
        super(context);
    }

    @Override
    public void surfaceCreated(@NonNull SurfaceHolder surfaceHolder) {
        mNativeCanvas = mRustBridge.createWgpuCanvas(surfaceHolder.getSurface(), mExampleIndex);
    }

    @Override
    public void surfaceChanged(@NonNull SurfaceHolder surfaceHolder, int i, int i1, int i2) {

    }

    @Override
    public void surfaceDestroyed(@NonNull SurfaceHolder surfaceHolder) {
        if (mNativeCanvas != Long.MAX_VALUE) {
            mRustBridge.dropWgpuCanvas(mNativeCanvas);
        }
    }

    public void changeExample(int index) {
        if (mNativeCanvas != Long.MAX_VALUE) {
            mRustBridge.changeExample(mNativeCanvas, index);
            mExampleIndex = index;
        }
    }
}
