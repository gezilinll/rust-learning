package com.example.wgpu;

import android.content.Context;
import android.graphics.Canvas;
import android.util.AttributeSet;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

import androidx.annotation.NonNull;

public class WGPUSurfaceView extends SurfaceView implements SurfaceHolder.Callback2 {
    private RustBridge mRustBridge = new RustBridge();
    private long mNativeCanvas = Long.MAX_VALUE;
    private int mExampleIndex = 0;

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
        mNativeCanvas = mRustBridge.createWgpuCanvas(surfaceHolder.getSurface(), mExampleIndex);
        setWillNotDraw(false);
    }

    @Override
    public void surfaceChanged(@NonNull SurfaceHolder surfaceHolder, int i, int i1, int i2) {
    }

    @Override
    public void draw(Canvas canvas) {
        super.draw(canvas);
        if (mNativeCanvas != Long.MAX_VALUE) {
            mRustBridge.enterFrame(mNativeCanvas);
        }
        invalidate();
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

    @Override
    public void surfaceRedrawNeeded(@NonNull SurfaceHolder holder) {

    }
}
