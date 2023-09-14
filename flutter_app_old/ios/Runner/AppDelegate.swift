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
        if let args = call.arguments as? Dictionary<String, Any>,
            let input = args["input"] as? String {
            DispatchQueue.global(qos: .userInitiated).async {
                let res = generate_passphrase_ffi(input)
                let s_res = String(cString: res!)
                generate_passphrase_ffi_release(UnsafeMutablePointer(mutating: res))
                DispatchQueue.main.sync {
                    result(s_res)
                }
            }
        } else {
            result(FlutterError.init(code: "bad args", message: nil, details: nil))
        }
    })
    
    GeneratedPluginRegistrant.register(with: self)
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }
}
