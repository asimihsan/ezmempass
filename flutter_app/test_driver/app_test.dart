/*
  References

  - https://github.com/dart-lang/test/blob/master/pkgs/test/README.md#writing-tests
*/

import 'dart:io';

import 'package:flutter_driver/flutter_driver.dart';
import 'package:test/test.dart';

void main() {
  group('Generate Password App', () {
    // First define the Finders and use them to locate widgets from the test suite. Note: the
    // Strings provded to the `byValueKey` method must be the same as the Strings used in Key
    // for each Widget.
    final passwordTextFinder = find.byValueKey("password");
    final buttonFinder = find.byValueKey("generate");

    FlutterDriver driver;

    // Connect to the Flutter driver before running any tests
    setUpAll(() async {
      driver = await FlutterDriver.connect();
      await driver.checkHealth();
    });

    // Close the connection to the driver after the tests have completed.
    tearDownAll(() async {
      if (driver != null) {
        await driver.close();
      }
    });

    test('starts with a password', () async {
      // Passwords take a while to generate, so we sleep a bit then check.
      sleep(Duration(seconds: 1));
      String password = await driver.getText(passwordTextFinder);
      expect(password, isNot(""));
    });

    test('clicking generates passwords', () async {
      String password1 = await driver.getText(passwordTextFinder);
      await driver.waitFor(buttonFinder);
      await driver.tap(buttonFinder);
      String password2 = await driver.getText(passwordTextFinder);
      expect(password2, isNot(""));
      expect(password2, isNot(password1));
    });
  });
}
