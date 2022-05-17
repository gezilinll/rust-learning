package com.example.wgpu;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.widget.RelativeLayout;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("wgpu_on_app");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        WGPUSurfaceView surfaceView = new WGPUSurfaceView(this);
        ((RelativeLayout) findViewById(R.id.rl_container)).addView(surfaceView);
    }
}