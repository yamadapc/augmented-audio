import UIKit
import Flutter
import AVFAudio

func configureAudioSession() {
  let audioSession = AVAudioSession.sharedInstance()
  do {
    // Set the audio session category and mode.
    try audioSession.setCategory(
      .playback,
      mode: .default,
      options: [.mixWithOthers, .allowBluetooth, .allowAirPlay]
    )
  } catch {
    print("Failed to set the audio session configuration")
  }
}

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    let value = dummy_method_to_enforce_bundling()
    print(value)

    configureAudioSession()

    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
