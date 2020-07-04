# ezmempass

Strong, easy to memorize passwords.

## Introduction

This document describes the project at a high level and how to get started developing with the Rust shared library and the Flutter mobile app.

-   To read more about how to generate the language model, see LANGUAGE_MODEL.md.

## What problem does this project solve?

Passwords are important and, for better or worse, aren't going anywhere. The best way of dealing with passwords is to use a password manager like 1Password to store random, unique passwords for each website, and then a single very strong password for the password manager. However there are two issues with this.

Firstly, what password do you choose for your password manager? Ultimately there must be a password somewhere. It is true that you could store a complex password on a device like a smart card, but then something "you have" like a smart card has different security and legal properties to something "you know" like a password.

Secondly, there are some Internet accounts that are so important, like email addresses or certain social media accounts, that just in case you forget your password manager password you simply can't lose access to. Moreover some secure locations prohibit access to password managers.

You will always need a small handful of strong passwords, i.e. passwords that are essentially impossible for an attacker to guess via brute force. However such passwords are very difficult to memorize. Based on my personal experience, strong passwords based on random characters take around 5 days to memorize if you use the password at least 3 times a day.

What if there was a way of creating passwords that were slightly longer and much easier to remember, but just as strong as a password made up of random characters? The scheme I describe below generates passwords that, based on my personal experience, take around 1-2 days to memorize.

## What is the proposed way of generating passwords?

First we need data:

-   Obtain a very large body of text in a given language, e.g. English. A [Wikipedia cirrussearch dump](https://dumps.wikimedia.org/other/cirrussearch/) is very handy here.
-   Come up with a way of iterating over all words in the body of text.
-   Determine the 100,000 most popular words in the data set that are also in a very large dictionary. This is the first pass.
-   Determine the 1024 most popular three-letter prefixes across the top 100,000 words. This is the second pass.
-   For each of the most popular words that also begins with one of the 1024 three-letter prefixes, track the frequence with which another of the most popular words occurs (i.e. get bigram statistics). This is the third pass.
-   Store
    -   The subset of the 100,000 most popular words that also begin with one of the 1024 most popular three-letter prefixes. This is around 54,000 words in English.
    -   For these 54,000 words, the bigram information.

Given this data and language model, we can generate strong relatively easy-to-memorize passwords:

-   Choose K three-letter prefixes at random.
-   Try to come up with a "plausible" sequence of words that match all of the three-letter prefixes.

The strength / entropy of this password is `8 * log_2(1024)` = `8 * 10` = `80` bits. As of 2019 this is a strong password. Moreover the phrase isn't a list of completely random words, instead we try to come up something that makes a little sense.

## What is the proposed technical solution?

Implement the data gathering and password generation code in Rust. Rust is a safe, fast, and concurrency-friendly language. Moreover, Rust is easy to cross-compile in a way that makes libraries available to a variety of desktop and mobile operating systems. Hence we can use a single Rust library to power a command-line interface, an iOS app, and an Android app.

TODO more details about the implementation.

We use Flutter to power the mobile apps because I don't want to learn how to make iOS / Android apps from scratch again.

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
-   Follow the "Test drive" step of the "Get started" instructions of the Flutter tutorial: https://flutter.dev/docs/get-started/test-drive
    -   This will ensure that at least an Android Virtual Device (AVD) is set up.
    -   If you're on Mac, also try to run the initial test app on an iOS Simulator.
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

-   Install the `cargo-ndk` Rust crate that lets you build Rust libraries for Android:

```
cargo install cargo-ndk
```

-   Install the `cbindgen` Rust crate that lets you create C header files quickly.

```
cargo install cbindgen
```

-   Install the Rust targets for iOS. This lets you cross-compile the Rust library so that it can run on iOS devices, even the Simulator.

```
rustup target add aarch64-apple-ios x86_64-apple-ios
```

-   Install the Rust targets for Android. This lets you cross-compile the Rust library so that it can run on Android devices, even the Android Virtual Device (AVD) emulator.

```
rustup target add aarch64-linux-android arm-linux-androideabi armv7-linux-androideabi i686-linux-android x86_64-linux-android
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

#### Java setup

We use Gradle as a build tool. Gradle runs a sequence of commands that builds the underlying Rust library, Flutter code, iOS and Android applications. In order to start using Gradle you need to install Java on your system:

-   Amazon Corretto is a fork of the OpenJDK version of Java, download and install version 8 of it: https://aws.amazon.com/corretto/. There are two important installation steps:
    1.  Download the correct installer format for your operating system and install it.
    2.  Go back to the root page https://aws.amazon.com/corretto/, follow the OS-specific "Installation Guide for Corretto 11" link at the bottom, and complete setting up your environment variables.

The Android command-line tools only support Java 8. You can have both 8 and 11 installed at the same time, just ensure the `JAVA_HOME` variable points at 8. On Mac you can run `/usr/libexec/java_home --verbose` to get the `JAVA_HOME` paths.

#### Building everything on Mac

This is the most typical interaction point on Mac systems using Gradle:

```
./gradlew clean all
```

For Windows you need to only run and build Android, see below.

Once built you can run on either iOS or Android by opening the `flutter_app` sub-folder in Android Studio then running the app.

### Setup just for Rust

This is the part that will work on all OS's. You can use a command-line utility that generates passwords for you. The library that powers this CLI utility is the same library used in the iOS and Android apps. You don't typically build or use the CLI tool, this is just a step to get used to Rust.

In this step we just use `cargo` directly. `cargo` is how you build Rust applications.

```
cd rust

# This is how you build.
cargo build

# This is how to run the unit tests
cargo test

# Here is how to run the CLI tool via cargo, an 8-word passphrase.
cargo run --bin ezmempass --release -- 8

# Here is how to run the CLI tool directlry - it's a static binary, self-contained!
target/release/ezmempass 8
```

### Setup for just Android

Note that Android command-line tools only work on Java 8, not Java 11!

If you're running on Windows then you can only develop Android apps, not iOS apps. First ensure that you have an Android Virtual Device (AVD) created for the Android target SDK version you want to test (for us it's 28). You can launch Android Studio, go to Tools -> AVD Manager, and create one.

To use Android command-line tools check your Android SDK path in `flutter doctor -v` and assuming the path is e.g. `/Users/asimi/Library/Android/sdk` ensure `/Users/asimi/Library/Android/sdk/tools/bin` is in your system path variable.

Then from the command line run `avdmanager list avd` to see a list of virtual devices and pick one that you want to use. In my case I want to use `Nexus_5X_API_27_x86`.

First run the emulator:

```
flutter emulators --launch Nexus_5X_API_27_x86
```

Then get the ID of the running emulator:

```
$ flutter devices
1 connected device:

Android SDK built for x86 • emulator-5554 • android-x86 • Android 8.1.0 (API 27) (emulator)
```

Here `emulator-5554` is the device ID. Finally run the app on the emulator, for Windows:

```
gradlew.bat runAndroid -PemulatorId=emulator-5554
```

For Mac:

```
./gradlew runAndroid -PemulatorId=emulator-5554
```

You can do the equivalent from the Android Studio, just open `flutter_app` in Android Studio, use the AVD manager to start a new emulator, then run the app on it.

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

## How to create privacy policy HTML file

-   Download `pandoc-goodies`: https://github.com/tajmone/pandoc-goodies/
-   Install `GitHub.html5` template to `~/.pandoc/templates`:

```
mkdir -p ~/.pandoc/templates
cp ~/Downloads/pandoc-goodies-master/templates/html5/github/GitHub.html5 ~/.pandoc/templates
```

-   Change to root directory of `ezmempass` then run:

```
pandoc --toc --output docs/privacy_policy/index.html --variable "pagetitle:EzMemPass Privacy Policy" --template=GitHub.html5 flutter_app/assets/privacy_policy.md
```

## How to deploy privacy policy HTML file

```
cd cdk
npm run build

# TODO add tests
# npm run test

# if this is the first time you've deployed to this AWS account
cdk bootstrap

npm run build && cdk deploy prod-EzMemPassPrivacyPolicyStack --strict
```

## How to create support site HTML file

Same prereqs as privacy policy, then

```
pandoc --toc --output docs/support_site/index.html --variable "pagetitle:EzMemPass" --template=GitHub.html5 docs/support_site.md
```


## How to deploy support site

```
cd cdk

npm run build && cdk deploy prod-EzMemPassSupportSiteStack --strict
```
