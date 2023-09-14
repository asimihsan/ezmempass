import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter_app/gen/protos/preferences.pb.dart';
import 'package:shared_preferences/shared_preferences.dart';

class PreferencesModel extends ChangeNotifier {
  // This is the key under which we store the preferences.
  static const PREFERENCES_KEY = "preferences";

  // Default preferences.
  static const DEFAULT_PASSPHRASE_LENGTH = 7;
  static const DEFAULT_ADD_CAPITAL_LETTER = false;
  static const DEFAULT_ADD_DIGIT = false;
  static const DEFAULT_ADD_SYMBOL = false;
  static const DEFAULT_IS_FIRST_LAUNCH = true;

  // This is the Protobuf we'd like the persist in storage.
  Preferences _preferences = new Preferences();

  // This is the plugin object used to persist perferences. Note that even initializing the object
  // is a Future.
  SharedPreferences _sharedPreferences;

  Future<void> reload() async {
    if (_sharedPreferences == null) {
      _sharedPreferences = await SharedPreferences.getInstance();
    }
    final String spSerialized = _sharedPreferences.getString("preferences");
    if (spSerialized != null) {
      final preferencesBytes = base64.decode(spSerialized);
      _preferences = Preferences.fromBuffer(preferencesBytes);
    }
  }

  Future<void> persist() async {
    final preferencesBytes = _preferences.writeToBuffer();
    final preferencesString = base64.encode(preferencesBytes);
    await _sharedPreferences.setString("preferences", preferencesString);
  }

  Future<void> clear() async {
    _preferences = new Preferences();
    await persist();
  }

  int get passphraseLength {
    if (_preferences.hasPassphraseLength()) {
      return _preferences.passphraseLength;
    }
    return DEFAULT_PASSPHRASE_LENGTH;
  }

  Future<void> setPassphraseLength(final int passphraseLength) async {
    _preferences.passphraseLength = passphraseLength;
    notifyListeners();
    persist();
  }

  bool get addCapitalLetter {
    if (_preferences.hasAddCapitalLetter()) {
      return _preferences.addCapitalLetter;
    }
    return DEFAULT_ADD_CAPITAL_LETTER;
  }

  Future<void> setAddCapitalLetter(final bool addCapitalLetter) async {
    _preferences.addCapitalLetter = addCapitalLetter;
    notifyListeners();
    persist();
  }

  bool get addDigit {
    if (_preferences.hasAddDigit()) {
      return _preferences.addDigit;
    }
    return DEFAULT_ADD_DIGIT;
  }

  Future<void> setAddDigit(final bool addDigit) async {
    _preferences.addDigit = addDigit;
    notifyListeners();
    persist();
  }

  bool get addSymbol {
    if (_preferences.hasAddSymbol()) {
      return _preferences.addSymbol;
    }
    return DEFAULT_ADD_SYMBOL;
  }

  Future<void> setAddSymbol(final bool addSymbol) async {
    _preferences.addSymbol = addSymbol;
    notifyListeners();
    persist();
  }

  bool get isFirstLaunch {
    if (_preferences.hasIsFirstLaunch()) {
      return _preferences.isFirstLaunch;
    }
    return DEFAULT_IS_FIRST_LAUNCH;
  }

  Future<void> setIsFirstLaunch(final bool isFirstLaunch) async {
    _preferences.isFirstLaunch = isFirstLaunch;
    notifyListeners();
    persist();
  }
}
