package com.example.rust;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.util.Log;
import android.view.View;

public class MainActivity extends AppCompatActivity implements JNICallback {

    static {
        System.loadLibrary("rust");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        findViewById(R.id.b_rust_hello).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                invokeCallbackViaJNI(MainActivity.this);
            }
        });
    }

    public static native void invokeCallbackViaJNI(JNICallback callback);

    @Override
    public void callback(String string) {
        Log.e("Rust", "From JNI: " + string + "\n");
    }
}