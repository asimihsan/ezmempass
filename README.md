# ezmempass

Strong, easy to memorize passwords.

## How to get started with development

### Key references

-   https://dev.to/robertohuertasm/rust-once-and-share-it-with-android-ios-and-flutter-286o

### Dependencies

-   Install Jetbrains Toolbox. Then within that install
    -   Android Studio
    -   (optional, costs money) CLion
-   Launch Android Studio, and install the Android SDK.
-   Install Flutter to the `$HOME/flutter` directory (your home directory): https://flutter.dev/docs/get-started/install
-   Ensure `flutter doctor` eventually runs without warnings or errors, follow its instructions.
-   Set up the Android NDK
    -   Download the ZIP file from https://developer.android.com/ndk/downloads/
    -   When you run `flutter doctor -v`, you will find out where the Android SDK is. For me it's at `/Users/asimi/Library/Android/sdk`
    -   Extract the ZIP file to a folder into a `ndk-bundle` subfolder, e.g. `/Users/asimi/Library/Android/sdk/ndk-bundle`
    -   Update your shell config e.g. `~/.zshrc` to include `export NDK_HOME=$HOME/Library/Android/sdk/ndk-bundle`, the path to your `ndk-bundle`
-   Install Rust: https://www.rust-lang.org/tools/install
-   Install the `cargo-lipo` Rust crate that lets you build Rust libraries for iOS:

```
cargo install cargo-lipo
```

-   Install the `cbindgen` Rust crate that lets you create C header files quickly.

```
cargo install cbindgen
```

-   Install the Rust targets for iOS:

```
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
```

-   Install the Rust targets for Android

```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

-   Install protobuf generator:

```
brew install protobuf
```

-   Install Dart plugin for the protoc compiler globally (reference: https://pub.dev/packages/protoc_plugin)

```
flutter pub global activate protoc_plugin

# then add `$HOME/.pub-cache/bin` in your home dir to your `PATH`, as the warning message says
```

-   Ensure the `dart` executable is in your PATH too, add following to `~/.zshrc`:

```
export PATH="${PATH}":"${HOME}"/flutter/bin/cache/dart-sdk/bin/
```

### Setup

-   Finally you can run the root build step, clean then build all.

```
./gradlew clean all
```

-   If this command fails some part of the setup doesn't work. We can iterate on the setup to improve it.
-   Subsequent builds you just need to do `all`.

### Troubleshooting

-   I don't think iOS builds to a device will work until you have the provisioning profile for `com.ezmempass.EzMemPass`
    -   TODO figure out how to share this.

### Running on real iOS device

-   From Android Studio, open the `ios\Runner\AppDelete.swift` file. This will reveal a link to "Open iOS module in Xcode", click that link.
-   If this is a fresh Catalina install there will be a lot of security warnings. Go to the Security preferences and allow all the executables flutter wants to run.

