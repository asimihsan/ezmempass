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
    });

    // Close the connection to the driver after the tests have completed.
    tearDownAll(() async {
      if (driver != null) {
        await driver.close();
      }
    });

    test('clicking generates passwords', () async {
      final Health health = await driver.checkHealth();
      print(health.status);

      String password = await driver.getText(passwordTextFinder);
      print(password);

      for (int i = 0; i < 10; i++) {
        await driver.waitFor(buttonFinder);
        await driver.tap(buttonFinder);
        String password = await driver.getText(passwordTextFinder);
        print(password);
      }
    });
  });
}
