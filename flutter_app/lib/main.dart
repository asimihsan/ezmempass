import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

void main() {
  runApp(MyApp());
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
  String _password = '';
  String _passphrase = '';
  int _passphraseLength = 7;
  bool _addCapitalLetter = false;
  bool _addDigit = false;
  bool _addSymbol = false;
  bool _onlyGeneratingOnce = true;
  bool _isAlreadyGenerating = false;

  Future<void> _generatePassphrase() async {
    if (_isAlreadyGenerating) {
      return;
    }
    _isAlreadyGenerating = true;
    String password;
    String passphrase;
    try {
      final Map<String, dynamic> inputMap = new Map();
      inputMap["passphrase_length"] = _passphraseLength;
      inputMap["add_capital_letter"] = _addCapitalLetter;
      inputMap["add_digit"] = _addDigit;
      inputMap["add_symbol"] = _addSymbol;
      final String input = jsonEncode(inputMap);
      final String result = await platform.invokeMethod(
          'generatePassphrase', {"input": input});
      final Map<String, dynamic> resultDeser = jsonDecode(result);
      password = resultDeser['password'];
      passphrase = resultDeser['passphrase'];
    } on PlatformException catch (e) {
      passphrase = "Failed to get passphrase: '${e.message}'.";
    }
    setState(() {
      _password = password;
      _passphrase = passphrase;
      _onlyGeneratingOnce = false;
      _isAlreadyGenerating = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_onlyGeneratingOnce) {
      _onlyGeneratingOnce = false;
      _generatePassphrase();
    }

    // ------------------------------------------------------------------------
    //  Password
    // ------------------------------------------------------------------------
    final Widget passwordLabel = Text(
      "Password",
      style: Theme.of(context).textTheme.headline.copyWith(
          fontFamily: 'RecursiveSansLinear'),
    );
    final Widget passwordValue = Expanded(
      child: SelectableText(
        _password,
        style: Theme.of(context).textTheme.title.copyWith(
            fontFamily: 'RecursiveMonoLinear'),
      )
    );
    final Widget passwordContainer = Container(
        padding: const EdgeInsets.all(32),
        child: Column(
          children: <Widget>[
            Row(
              children: <Widget>[
                passwordLabel,
              ],
            ),
            Row(
              children: <Widget>[
                passwordValue
              ],
            )
          ],
        )
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Passphrase
    // ------------------------------------------------------------------------
    final Widget passphraseLabel = Text(
      "Memory aid",
      style: Theme.of(context).textTheme.headline.copyWith(
          fontFamily: 'RecursiveSansLinear'),
    );
    final Widget passphraseValue = Expanded(
      child: SelectableText(
        _passphrase,
        style: Theme.of(context).textTheme.title.copyWith(
            fontFamily: 'RecursiveMonoLinear'),
      )
    );
    final Widget passphraseContainer = Container(
        padding: const EdgeInsets.fromLTRB(32, 0, 32, 32),
        child: Column(
          children: <Widget>[
            Row(
              children: <Widget>[
                passphraseLabel,
              ],
            ),
            Row(
              children: <Widget>[
                passphraseValue
              ],
            )
          ],
        )
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Number of words slider.
    // ------------------------------------------------------------------------
    final Widget numberOfWordsLabel = Text(
      "Number of words",
      style: Theme.of(context).textTheme.headline.copyWith(
          fontFamily: 'RecursiveSansLinear'),
    );
    final numberOfWordsSelector = DropdownButton<int>(
      value: _passphraseLength,
      icon: Icon(Icons.arrow_downward),
      iconSize: 24,
      elevation: 16,
      style: Theme.of(context).textTheme.headline.copyWith(
          fontFamily: 'RecursiveSansLinear'),
      underline: Container(
        height: 2,
        color: Colors.grey,
      ),
      onChanged: (int newPassphraseLength) {
        if (newPassphraseLength == _passphraseLength) {
          return;
        }
        setState(() {
          _onlyGeneratingOnce = true;
          _passphraseLength = newPassphraseLength;
        });
      },
      items: <int>[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                   21, 22, 23, 24].map<DropdownMenuItem<int>>((int value) {
        return DropdownMenuItem<int>(
          value: value,
          child: Text(value.toString()),
        );
      }).toList(growable: false),
    );
    final Widget numberOfWordsContainer = Container(
        padding: const EdgeInsets.fromLTRB(32, 0, 64, 8),
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
        )
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add capital letter switch
    // ------------------------------------------------------------------------
    final Widget capitalLetterSwitch = SwitchListTile(
      title: Text(
          'Capital letters',
          style: Theme.of(context).textTheme.headline.copyWith(
              fontFamily: 'RecursiveSansLinear')),
      value: _addCapitalLetter,
      onChanged: (bool value) {
        setState(() {
          _addCapitalLetter = value;
          _onlyGeneratingOnce = true;
        });
      }
    );
    final Widget capitalLetterContainer = Container(
      padding: const EdgeInsets.fromLTRB(16, 0, 32, 0),
      child: capitalLetterSwitch
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add digit switch
    // ------------------------------------------------------------------------
    final Widget digitSwitch = SwitchListTile(
        title: Text(
            'Digit',
            style: Theme.of(context).textTheme.headline.copyWith(
                fontFamily: 'RecursiveSansLinear')),
        value: _addDigit,
        onChanged: (bool value) {
          setState(() {
            _addDigit = value;
            _onlyGeneratingOnce = true;
          });
        }
    );
    final Widget digitContainer = Container(
        padding: const EdgeInsets.fromLTRB(16, 0, 32, 0),
        child: digitSwitch
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Add symbol
    // ------------------------------------------------------------------------
    final Widget symbolSwitch = SwitchListTile(
        title: Text(
            'Symbol',
            style: Theme.of(context).textTheme.headline.copyWith(
                fontFamily: 'RecursiveSansLinear')),
        value: _addSymbol,
        onChanged: (bool value) {
          setState(() {
            _addSymbol = value;
            _onlyGeneratingOnce = true;
          });
        }
    );
    final Widget symbolContainer = Container(
        padding: const EdgeInsets.fromLTRB(16, 0, 32, 0),
        child: symbolSwitch
    );
    // ------------------------------------------------------------------------


    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
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
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          setState(() {
            _onlyGeneratingOnce = true;
          });
        },
        tooltip: 'Generate',
        child: Icon(Icons.add),
      ),
    );
  }
}
