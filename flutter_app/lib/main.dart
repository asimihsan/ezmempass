import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_app/initial_intro.dart';
import 'package:flutter_app/models/preferences_model.dart';
import 'package:flutter_app/widgets/password_generator_app.dart';
import 'package:provider/provider.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final PreferencesModel preferencesModel = new PreferencesModel();
  await preferencesModel.reload();
  if (preferencesModel.isFirstLaunch) {
    runApp(ListenableProvider(create: (_) => preferencesModel, child: InitialIntro()));
  } else {
    runApp(ListenableProvider(create: (_) => preferencesModel, child: PasswordGeneratorApp()));
  }
}
