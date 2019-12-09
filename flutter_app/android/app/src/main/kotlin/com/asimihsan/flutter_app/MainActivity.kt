package com.asimihsan.flutter_app

import android.os.Bundle
import android.os.Handler
import android.os.Looper

import io.flutter.app.FlutterActivity
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugins.GeneratedPluginRegistrant
import java.util.concurrent.Executors

class MainActivity: FlutterActivity() {
  private val GENERATE_PASSPHRASE_CHANNEL = "com.asimihsan/generate_passphrase"
  private val executorService = Executors.newSingleThreadExecutor();

  override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    GeneratedPluginRegistrant.registerWith(this)

    loadGeneratePassphraseLib()
    MethodChannel(flutterView, GENERATE_PASSPHRASE_CHANNEL).setMethodCallHandler { call, result ->
      if (call.method == "generatePassphrase") {
        val input: String? = call.argument<String>("input")
        if (input == null) {
          result.error("UNAVAILABLE", "input is null", null)
        } else {
          // Flutter platform channel is currently running in the main UI thread. Password
          // generation is slow so we don't want to block the UI thread for too long. Hence run the
          // actual Rust code in a separate thread, but then call the "result.success" method back
          // on the main UI thread as Flutter requires.
          executorService.submit {
              val generatePassphraseResult = generatePassphrase(input)
              Handler(Looper.getMainLooper()).post {
                result.success(generatePassphraseResult);
              }
          }
        }
      } else {
        result.notImplemented()
      }
    }
  }
}
