package com.beijaflor.metronome

import android.os.Bundle
import android.util.Log
import io.flutter.embedding.android.FlutterActivity
import java.io.File
import java.io.FileOutputStream

class MainActivity: FlutterActivity() {
    init {
        // We need to call loadLibrary before dart so that JNI_OnLoad gets called
        System.loadLibrary("metronome")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        val result = assets.list("sounds")
        result?.forEach { sound ->
            val soundFile = assets.open("sounds/$sound")
            val internalFile = File(context.filesDir, "$sound")
            val outputStream = FileOutputStream(internalFile)

            val buffer = ByteArray(1024)
            var read: Int
            while (soundFile.read(buffer).also { read = it } != -1) {
                outputStream.write(buffer, 0, read)
            }

            Log.i("metronome", "Copied sounds/$sound to ${internalFile.absolutePath}")
            soundFile.close()
            outputStream.flush()
            outputStream.close()
        }

        super.onCreate(savedInstanceState)
    }
}
