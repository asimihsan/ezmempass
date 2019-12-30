import 'dart:convert';
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_app/help_screen.dart';
import 'package:flutter_app/initial_intro.dart';
import 'package:flutter_app/multiplatform_switch_list_tile.dart';
import 'package:flutter_app/gen/protos/preferences.pb.dart';
import 'package:share/share.dart';
import 'package:shared_preferences/shared_preferences.dart';

void main() {
  runApp(InitialIntro());
  //runApp(MyApp());
}

class MyApp extends StatelessWidget {
  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'EzMemPass',
      theme: ThemeData(
        brightness: Brightness.light,
        primarySwatch: Colors.blue,
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'EzMemPass'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key key, this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  static const platform = const MethodChannel('com.asimihsan/generate_passphrase');
  SharedPreferences sharedPreferences;

  String _password = '';
  String _passphrase = '';
  int _passphraseLength = 7;
  bool _addCapitalLetter = false;
  bool _addDigit = false;
  bool _addSymbol = false;

  Future<void> _generatePassphrase() async {
    String password;
    String passphrase;
    try {
      final Map<String, dynamic> inputMap = new Map();
      inputMap["passphrase_length"] = _passphraseLength;
      inputMap["add_capital_letter"] = _addCapitalLetter;
      inputMap["add_digit"] = _addDigit;
      inputMap["add_symbol"] = _addSymbol;
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

    SharedPreferences.getInstance().then((SharedPreferences sp) {
      int passphraseLength = 7;
      bool addCapitalLetter = false;
      bool addDigit = false;
      bool addSymbol = false;

      sharedPreferences = sp;
      String spSerialized = sharedPreferences.getString("preferences");
      if (spSerialized != null) {
        try {
          final preferencesBytes = base64.decode(spSerialized);
          final preferences = Preferences.fromBuffer(preferencesBytes);
          if (preferences.hasPassphraseLength()) {
            passphraseLength = preferences.passphraseLength;
          }
          if (preferences.hasAddCapitalLetter()) {
            addCapitalLetter = preferences.addCapitalLetter;
          }
          if (preferences.hasAddDigit()) {
            addDigit = preferences.addDigit;
          }
          if (preferences.hasAddSymbol()) {
            addSymbol = preferences.addSymbol;
          }
        } catch (e) {
          // Any failure to load preferences is OK, we ignore it and set defaults.
        }
      }
      setState(() {
        _passphraseLength = passphraseLength;
        _addCapitalLetter = addCapitalLetter;
        _addDigit = addDigit;
        _addSymbol = addSymbol;
      });
      _generatePassphrase();
      persistPreferences();
    });
  }

  void persistPreferences() {
    final Preferences preferences = new Preferences();
    preferences.passphraseLength = _passphraseLength;
    preferences.addCapitalLetter = _addCapitalLetter;
    preferences.addDigit = _addDigit;
    preferences.addSymbol = _addSymbol;
    final preferencesBytes = preferences.writeToBuffer();
    final preferencesString = base64.encode(preferencesBytes);
    sharedPreferences.setString("preferences", preferencesString);
  }

  @override
  Widget build(BuildContext context) {
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
      style: Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear'),
    );
    final Widget passwordValue = Expanded(
        child: Text(
      _password,
      style: Theme.of(context).textTheme.title.copyWith(fontFamily: 'RecursiveMonoLinear'),
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
      style: Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear'),
    );
    final Widget passphraseValue = Expanded(
        child: Text(
      _passphrase,
      style: Theme.of(context).textTheme.title.copyWith(fontFamily: 'RecursiveMonoLinear'),
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
      style: Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear'),
    );
    final numberOfWordsSelector = DropdownButton<int>(
      value: _passphraseLength,
      icon: Icon(Icons.arrow_downward),
      iconSize: 24,
      elevation: 16,
      style: Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear'),
      underline: Container(
        height: 2,
        color: Colors.grey,
      ),
      onChanged: (int newPassphraseLength) {
        if (newPassphraseLength == _passphraseLength) {
          return;
        }
        setState(() {
          _passphraseLength = newPassphraseLength;
        });
        _generatePassphrase();
        persistPreferences();
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
            style:
                Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear')),
        value: _addCapitalLetter,
        onChanged: (bool value) {
          setState(() {
            _addCapitalLetter = value;
          });
          _generatePassphrase();
          persistPreferences();
        });
    final Widget capitalLetterContainer =
        Container(padding: const EdgeInsets.fromLTRB(0, 8, 0, 0), child: capitalLetterSwitch);
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add digit switch
    // ------------------------------------------------------------------------

    final Widget digitSwitch = SwitchListTile(
        title: Text('Digit',
            style:
                Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear')),
        value: _addDigit,
        onChanged: (bool value) {
          setState(() {
            _addDigit = value;
          });
          _generatePassphrase();
          persistPreferences();
        });
    final Widget digitContainer =
        Container(padding: const EdgeInsets.fromLTRB(0, 0, 0, 0), child: digitSwitch);
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add symbol
    // ------------------------------------------------------------------------
    final Widget symbolSwitch = SwitchListTile(
        title: Text('Symbol',
            style:
                Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear')),
        value: _addSymbol,
        onChanged: (bool value) {
          setState(() {
            _addSymbol = value;
          });
          _generatePassphrase();
          persistPreferences();
        });
    final Widget symbolContainer =
        Container(padding: const EdgeInsets.fromLTRB(0, 0, 0, 0), child: symbolSwitch);
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add button
    // ------------------------------------------------------------------------
    final Widget button = new RaisedButton(
      padding: const EdgeInsets.all(16.0),
      onPressed: _generatePassphrase,
      child: new Text("Generate password",
          style: Theme.of(context).textTheme.headline.copyWith(fontFamily: 'RecursiveSansLinear')),
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
      ),
      body: ListView(
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
    );
  }
}
