import 'dart:convert';
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_app/widgets/help_screen.dart';
import 'package:flutter_app/models/preferences_model.dart';
import 'package:flutter_app/widgets/settings_widget.dart';
import 'package:provider/provider.dart';
import 'package:share/share.dart';

class PasswordGeneratorWidget extends StatefulWidget {
  PasswordGeneratorWidget({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _PasswordGeneratorWidgetState createState() => _PasswordGeneratorWidgetState();
}

class _PasswordGeneratorWidgetState extends State<PasswordGeneratorWidget> {
  static const platform = const MethodChannel('com.asimihsan/generate_passphrase');

  String _password = '';
  String _passphrase = '';

  Future<void> _generatePassphrase(final PreferencesModel preferencesModel) async {
    String password;
    String passphrase;
    try {
      final Map<String, dynamic> inputMap = new Map();
      inputMap["passphrase_length"] = preferencesModel.passphraseLength;
      inputMap["add_capital_letter"] = preferencesModel.addCapitalLetter;
      inputMap["add_digit"] = preferencesModel.addDigit;
      inputMap["add_symbol"] = preferencesModel.addSymbol;
      final String input = jsonEncode(inputMap);
      final String result = await platform.invokeMethod('generatePassphrase', {"input": input});
      final Map<String, dynamic> resultDeser = jsonDecode(result);
      password = resultDeser['password'];
      passphrase = resultDeser['passphrase'];
    } on PlatformException catch (e) {
      passphrase = "Failed to get passphrase: '${e.message}'.";
    }
    setState(() {
      _password = password;
      _passphrase = passphrase;
    });
  }

  @override
  void initState() {
    super.initState();
    new Future.delayed(Duration.zero, () {
      final PreferencesModel preferencesModel =
          Provider.of<PreferencesModel>(context, listen: false);
      _generatePassphrase(preferencesModel);
    });
  }

  @override
  Widget build(final BuildContext context) {
    return Consumer<PreferencesModel>(builder: (context, preferencesModel, child) {
      // ------------------------------------------------------------------------
      //  Shared widgets
      // ------------------------------------------------------------------------
      Icon shareIcon;
      if (Platform.isAndroid) {
        shareIcon = Icon(Icons.content_paste, size: 44);
      } else {
        shareIcon = Icon(CupertinoIcons.share, size: 44);
      }
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Password
      // ------------------------------------------------------------------------
      final Widget passwordLabel = Text(
        "Password",
        style: Theme.of(context).textTheme.headline5.copyWith(fontFamily: 'RecursiveSansLinear'),
      );
      final Widget passwordValue = Expanded(
          child: Text(
        _password,
        style: Theme.of(context).textTheme.headline6.copyWith(fontFamily: 'RecursiveMonoLinear'),
        key: Key('password'),
      ));
      final Widget passwordValueCopyButton = IconButton(
        icon: shareIcon,
        alignment: Alignment.topCenter,
        padding: const EdgeInsets.all(0),
        onPressed: () {
          Share.share(_password);
        },
      );
      final Widget passwordContainer = Container(
          padding: const EdgeInsets.all(16),
          child: Column(
            children: <Widget>[
              Row(
                children: <Widget>[
                  passwordLabel,
                ],
              ),
              Row(
                children: <Widget>[passwordValue, passwordValueCopyButton],
              )
            ],
          ));
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Passphrase
      // ------------------------------------------------------------------------
      final Widget passphraseLabel = Text(
        "Memory aid",
        style: Theme.of(context).textTheme.headline5.copyWith(fontFamily: 'RecursiveSansLinear'),
      );
      final Widget passphraseValue = Expanded(
          child: Text(
        _passphrase,
        style: Theme.of(context).textTheme.headline6.copyWith(fontFamily: 'RecursiveMonoLinear'),
      ));
      final Widget passphraseValueCopyButton = IconButton(
        icon: shareIcon,
        alignment: Alignment.topCenter,
        padding: const EdgeInsets.all(0),
        onPressed: () {
          Share.share(_passphrase);
        },
      );
      final Widget passphraseContainer = Container(
          padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
          child: Column(
            children: <Widget>[
              Row(
                children: <Widget>[
                  passphraseLabel,
                ],
              ),
              Row(
                children: <Widget>[passphraseValue, passphraseValueCopyButton],
              )
            ],
          ));
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Number of words selector.
      // ------------------------------------------------------------------------
      final Widget numberOfWordsLabel = Text(
        "Number of words",
        style: Theme.of(context).textTheme.headline5.copyWith(fontFamily: 'RecursiveSansLinear'),
      );
      final numberOfWordsSelector = DropdownButton<int>(
        value: preferencesModel.passphraseLength,
        icon: Icon(Icons.arrow_downward),
        iconSize: 24,
        elevation: 16,
        style: Theme.of(context).textTheme.headline5.copyWith(fontFamily: 'RecursiveSansLinear'),
        underline: Container(
          height: 2,
          color: Colors.grey,
        ),
        onChanged: (int newPassphraseLength) {
          if (newPassphraseLength == preferencesModel.passphraseLength) {
            return;
          }
          preferencesModel.setPassphraseLength(newPassphraseLength);
          _generatePassphrase(preferencesModel);
        },
        items: <int>[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]
            .map<DropdownMenuItem<int>>((int value) {
          return DropdownMenuItem<int>(
            value: value,
            child: Text(value.toString()),
          );
        }).toList(growable: false),
      );
      final Widget numberOfWordsContainer = Container(
          padding: const EdgeInsets.fromLTRB(16, 0, 32, 0),
          child: Column(
            children: <Widget>[
              Row(
                children: <Widget>[
                  numberOfWordsLabel,
                  Spacer(),
                  numberOfWordsSelector,
                ],
              ),
            ],
          ));
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Add capital letter switch
      // ------------------------------------------------------------------------
      final Widget capitalLetterSwitch = SwitchListTile(
          title: Text('Capital letters',
              style: Theme.of(context)
                  .textTheme
                  .headline5
                  .copyWith(fontFamily: 'RecursiveSansLinear')),
          value: preferencesModel.addCapitalLetter,
          onChanged: (bool value) {
            preferencesModel.setAddCapitalLetter(value);
            _generatePassphrase(preferencesModel);
          });
      final Widget capitalLetterContainer =
          Container(padding: const EdgeInsets.fromLTRB(0, 8, 0, 0), child: capitalLetterSwitch);
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Add digit switch
      // ------------------------------------------------------------------------
      final Widget digitSwitch = SwitchListTile(
          title: Text('Digit',
              style: Theme.of(context)
                  .textTheme
                  .headline5
                  .copyWith(fontFamily: 'RecursiveSansLinear')),
          value: preferencesModel.addDigit,
          onChanged: (bool value) {
            preferencesModel.setAddDigit(value);
            _generatePassphrase(preferencesModel);
          });
      final Widget digitContainer =
          Container(padding: const EdgeInsets.fromLTRB(0, 0, 0, 0), child: digitSwitch);
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Add symbol
      // ------------------------------------------------------------------------
      final Widget symbolSwitch = SwitchListTile(
          title: Text('Symbol',
              style: Theme.of(context)
                  .textTheme
                  .headline5
                  .copyWith(fontFamily: 'RecursiveSansLinear')),
          value: preferencesModel.addSymbol,
          onChanged: (bool value) {
            preferencesModel.setAddSymbol(value);
            _generatePassphrase(preferencesModel);
          });
      final Widget symbolContainer =
          Container(padding: const EdgeInsets.fromLTRB(0, 0, 0, 0), child: symbolSwitch);
      // ------------------------------------------------------------------------

      // ------------------------------------------------------------------------
      //  Add button
      // ------------------------------------------------------------------------
      final Widget button = new RaisedButton(
        padding: const EdgeInsets.all(16.0),
        onPressed: () {
          _generatePassphrase(preferencesModel);
        },
        child: new Text("Generate password",
            style:
                Theme.of(context).textTheme.headline5.copyWith(fontFamily: 'RecursiveSansLinear')),
        key: Key('generate'),
      );
      final Widget buttonContainer = Container(padding: const EdgeInsets.all(16), child: button);
      // ------------------------------------------------------------------------

      return Scaffold(
          appBar: AppBar(
            title: Text(widget.title),
            leading: IconButton(
              icon: Icon(Icons.help),
              onPressed: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => HelpScreen()),
                );
              },
            ),
            actions: <Widget>[
              IconButton(
                  icon: Icon(Icons.settings),
                  onPressed: () {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                          builder: (context) => ListenableProvider.value(
                              value: preferencesModel, child: SettingsWidget())),
                    );
                  }),
            ],
          ),
          body: SafeArea(
            child: ListView(
              physics: ClampingScrollPhysics(),
              children: <Widget>[
                passwordContainer,
                passphraseContainer,
                numberOfWordsContainer,
                capitalLetterContainer,
                digitContainer,
                symbolContainer,
                buttonContainer,
              ],
            ),
          ));
    });
  }
}
