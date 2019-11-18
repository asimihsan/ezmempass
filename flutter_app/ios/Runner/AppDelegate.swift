import UIKit
import Flutter

@UIApplicationMain
@objc class AppDelegate: FlutterAppDelegate {
  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    
    let controller : FlutterViewController = window?.rootViewController as! FlutterViewController
    let generatePassphraseChannel = FlutterMethodChannel(
        name: "com.asimihsan/generate_passphrase",
        binaryMessenger: controller.binaryMessenger)
    generatePassphraseChannel.setMethodCallHandler({
        (call: FlutterMethodCall, result: @escaping FlutterResult) -> Void in
        let res = generate_passphrase_ffi("input")
        let s_res = String(cString: res!)
        generate_passphrase_ffi_release(UnsafeMutablePointer(mutating: res))
        result(s_res)
    })
    
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
