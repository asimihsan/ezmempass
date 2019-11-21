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

  Future<void> _generatePassphrase() async {
    String password;
    String passphrase;
    try {
      final Map<String, dynamic> inputMap = new Map();
      inputMap["passphrase_length"] = _passphraseLength;
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
    });
  }

  @override
  Widget build(BuildContext context) {
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
            fontFamily: 'RecursiveMonoCasual'),
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
      "Passphrase",
      style: Theme.of(context).textTheme.headline.copyWith(
          fontFamily: 'RecursiveSansLinear'),
    );
    final Widget passphraseValue = Expanded(
      child: SelectableText(
        _passphrase,
        style: Theme.of(context).textTheme.title.copyWith(
            fontFamily: 'RecursiveMonoCasual'),
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
    final double minPassphraseLength = 4.0;
    final double maxPassphraseLength = 25.0;
    final int divisions = (maxPassphraseLength - minPassphraseLength).round() + 1;
    final Widget numberOfWordsSlider = Flexible(
      flex: 1,
      child: Slider(
        min: 4.0,
        max: 25.0,
        value: _passphraseLength.toDouble(),
        divisions: divisions,
        onChanged: (newPassphraseLength) {
          setState(() {
            _passphraseLength = newPassphraseLength.toInt();
          });
        },
      )
    );
    final Widget numberOfWordsSliderValue = Container(
      width: 50.0,
      child: Text(
        '$_passphraseLength',
        style: Theme.of(context).textTheme.title,
      )
    );
    final Widget numberOfWordsContainer = Container(
        padding: const EdgeInsets.fromLTRB(32, 0, 32, 32),
        child: Column(
          children: <Widget>[
            Row(
              children: <Widget>[
                numberOfWordsLabel
              ],
            ),
            Row(
              children: <Widget>[
                numberOfWordsSlider,
                numberOfWordsSliderValue
              ],
            ),
          ],
        )
    );
    // --------------
    // ------------------------------------------------------------------------

    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Container(
        child: Column(
          children: <Widget>[
            passwordContainer,
            passphraseContainer,
            numberOfWordsContainer,
          ],
        )
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _generatePassphrase,
        tooltip: 'Generate',
        child: Icon(Icons.add),
      ),
    );
  }
}
