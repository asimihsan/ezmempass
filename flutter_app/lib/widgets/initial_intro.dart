import 'package:carousel_slider/carousel_slider.dart';
import 'package:dots_indicator/dots_indicator.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_app/main.dart';
import 'package:flutter_app/models/preferences_model.dart';
import 'package:flutter_app/widgets/initial_intro_card.dart';
import 'package:flutter_app/widgets/password_generator_app.dart';
import 'package:provider/provider.dart';

class InitialIntro extends StatefulWidget {
  @override
  _InitialIntroState createState() => _InitialIntroState();
}

class _InitialIntroState extends State<InitialIntro> {
  static final EN_CARD_1 = """EzMemPass creates strong, easy to remember passwords!""";
  static final EN_CARD_2 = """Passwords will look like:

foobanmadava

There is a corresponding "memory aid" that is easier to memorize, like:

food bank made available

Remember the memory aid words and the first three letters of each word is your password!""";

  static final EN_CARD_3 = """Use passwords with 8 or more words to create strong passwords.""";

  static final NUMBER_OF_CARDS = 3;

  int _current = 0;

  @override
  Widget build(BuildContext context) {
    final Widget slider = Builder(builder: (BuildContext context) {
      return CarouselSlider(
        options: CarouselOptions(
            initialPage: 0,
            height: 2000,
            viewportFraction: 1.0,
            scrollDirection: Axis.horizontal,
            enableInfiniteScroll: false,
            onPageChanged: (index, reason) {
              setState(() {
                _current = index;
              });
            }),
        items: <Widget>[
          InitialIntroCard(cardText: EN_CARD_1),
          InitialIntroCard(cardText: EN_CARD_2),
          InitialIntroCard(cardText: EN_CARD_3),
        ],
      );
    });

    final Widget dotsIndicator = new DotsIndicator(
      dotsCount: NUMBER_OF_CARDS,
      position: _current.toDouble(),
    );

    final Widget sliderContainer = Builder(builder: (BuildContext context) {
      return Column(children: <Widget>[
        Expanded(
          child: Row(children: <Widget>[
            Expanded(
              child: slider,
            )
          ]),
        ),
        Container(
          padding: const EdgeInsets.all(10),
          child: dotsIndicator,
        ),
        RaisedButton(
            onPressed: () {
              final PreferencesModel preferencesModel =
                  Provider.of<PreferencesModel>(context, listen: false);
              preferencesModel.setIsFirstLaunch(false);
              final Widget child = ListenableProvider.value(
                value: preferencesModel,
                child: PasswordGeneratorApp(),
              );
              Navigator.pushReplacement(context, MaterialPageRoute(builder: (context) => child));
            },
            child: Text('Continue to the app'))
      ]);
    });
    return MaterialApp(
      theme: ThemeData(
        brightness: Brightness.light,
        primarySwatch: Colors.blue,
      ),
      darkTheme: ThemeData(
        brightness: Brightness.dark,
        primarySwatch: Colors.blue,
      ),
      home: SafeArea(
        child: sliderContainer,
      ),
    );
  }
}
