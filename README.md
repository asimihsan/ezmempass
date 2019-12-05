## How to get started with development

### Key references

-   https://dev.to/robertohuertasm/rust-once-and-share-it-with-android-ios-and-flutter-286o

### Dependencies

-   Install Jetbrains Toolbox. Then within that install
    -   Android Studio
    -   (optional, costs money) CLion
-   Install Flutter to the `$HOME/flutter` directory (your home directory): https://flutter.dev/docs/get-started/install
-   Ensure `flutter doctor` eventually runs without warnings or errors, follow its instructions.
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

-   Until this is properly version controlled in Git, delete the following dirs and files
    -   `flutter_app/ios/.symlinks`
    -   `flutter_app/.packages`
    -   `flutter_app/.flutter-plugins`
-   (Is this needed?) from the root of `flutter_app` run:

```
flutter pub get
```

-   Finally you can run the root build step:

```
./gradlew clean all
```

### Running on real iOS device

-   From Android Studio, open the `ios\Runner\AppDelete.swift` file. This will reveal a link to "Open iOS module in Xcode", click that link.
-   If this is a fresh Catalina install there will be a lot of security warnings. Go to the Security preferences and allow all the executables flutter wants to run.