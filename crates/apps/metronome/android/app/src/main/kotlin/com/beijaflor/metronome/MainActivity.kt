package com.beijaflor.metronome

import io.flutter.embedding.android.FlutterActivity

class MainActivity: FlutterActivity() {
    init {
        // We need to call loadLibrary before dart so that JNI_OnLoad gets called
        System.loadLibrary("metronome")
    }
}
