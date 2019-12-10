# ezmempass

Strong, easy to memorize passwords.

## How to get started with development

If you're on Windows you should still be able to build the Rust + Android parts of this project.

If you're on Mac you can build the Rust + Android + iOS parts of this project.

### Key references

-   https://dev.to/robertohuertasm/rust-once-and-share-it-with-android-ios-and-flutter-286o

### Dependencies

-   Install Jetbrains Toolbox. Then within that install
    -   Android Studio
    -   (optional, costs money) CLion
-   Launch Android Studio, and install the Android SDK.
    -   This should also create an Android Virtual Device (AVD) for you, if not create one that supports SDK 23 and higher.
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

-   Install the Rust targets for iOS. This lets you cross-compile the Rust library so that it can run on iOS devices, even the Simulator.

```
rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
```

-   Install the Rust targets for Android. This lets you cross-compile the Rust library so that it can run on Android devices, even the Android Virtual Device (AVD) emulator.

```
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

-   Install protobuf generator. We use protobufs for persisting user preferences.

```
brew install protobuf
```

-   Install Dart plugin for the protoc compiler globally (reference: https://pub.dev/packages/protoc_plugin). This is needed for us to generate Dart code from proto files.

```
flutter pub global activate protoc_plugin

# then add `$HOME/.pub-cache/bin` in your home dir to your `PATH`, as the warning message says
```

-   Ensure the `dart` executable is in your PATH too, add following to `~/.zshrc`:

```
export PATH="${PATH}":"${HOME}"/flutter/bin/cache/dart-sdk/bin/
```

### Setup for both iOS and Android

We use Gradle as a build tool. Gradle runs a sequence of commands that builds the underlying Rust library, Flutter code, iOS and Android applications. This is the most typical interaction point on Mac systems.

```
./gradlew clean all
```

-   If this command fails some part of the setup doesn't work. We can iterate on the setup to improve it.
-   Subsequent builds you just need to do `all`.


### Setup just for Rust

This is the part that will work on all OS's. You can use a command-line utility that generates passwords for you. The library that powers this CLI utility is the same library used in the iOS and Android apps. You don't typically build or use the CLI tool, this is just a step to get used to Rust.

In this step we just use `cargo` directly. `cargo` is how you build Rust applications.

```
cd rust

# This is how you build.
cargo build --workspace --all-targets --release

# This is how to run the unit tests
cargo test --workspace --all-targets --release

# Here is how to run the CLI tool via cargo, an 8-word passphrase.
cargo run --bin ezmempass --release -- 8

# Here is how to run the CLI tool directlry - it's a static binary, self-contained!
target/release/ezmempass 8
```

### Setup for just Android

-   If you just want to do the Android build because e.g. you're on a Windows machine you can instead run:

```
./gradlew flutterBuildAndroidFatApk
```

-   Then you want to use either Android Studio, open the Flutter project, and run on the Android Virtual Device (AVD), or do so from the command line. First launch the AVD simulator via Android Studio, Tools -> AVD Manager. Then:

```
cd flutter_app
flutter run -d android
```

### Troubleshooting

-   I don't think iOS builds to a device will work until you have the provisioning profile for `com.ezmempass.EzMemPass`
    -   TODO figure out how to share this.

### Running on real iOS device

-   From Android Studio, open the `ios\Runner\AppDelete.swift` file. This will reveal a link to "Open iOS module in Xcode", click that link.
-   If this is a fresh Catalina install there will be a lot of security warnings. Go to the Security preferences and allow all the executables flutter wants to run.

### Running the Android build on AWS Device Farm

TODO this is easy and doesn't require a special certificate.

### Running the iOS build on AWS Device Farm

TODO this is trickier and does require you to have the Apple release profile, or pay $99 for one.