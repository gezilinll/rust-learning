package com.example.wgpu;

import android.content.Context;
import android.util.AttributeSet;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

import androidx.annotation.NonNull;

public class WGPUSurfaceView extends SurfaceView implements SurfaceHolder.Callback {
    private RustBridge mRustBridge = new RustBridge();

    public WGPUSurfaceView(Context context) {
        this(context, null);
    }

    public WGPUSurfaceView(Context context, AttributeSet attrs) {
        this(context, attrs, 0);
    }

    public WGPUSurfaceView(Context context, AttributeSet attrs, int defStyleAttr) {
        this(context, attrs, defStyleAttr, 0);
    }

    public WGPUSurfaceView(Context context, AttributeSet attrs, int defStyleAttr, int defStyleRes) {
        super(context, attrs, defStyleAttr, defStyleRes);
        getHolder().addCallback(this);
    }


    @Override
    public void surfaceCreated(@NonNull SurfaceHolder surfaceHolder) {
        mRustBridge.startRenderTriangle(surfaceHolder.getSurface());
    }

    @Override
    public void surfaceChanged(@NonNull SurfaceHolder surfaceHolder, int i, int i1, int i2) {
    }

    @Override
    public void surfaceDestroyed(@NonNull SurfaceHolder holder) {

    }

}
