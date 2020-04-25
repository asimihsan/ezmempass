import 'package:carousel_slider/carousel_slider.dart';
import 'package:dots_indicator/dots_indicator.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_app/main.dart';
import 'package:flutter_app/widgets/initial_intro_card.dart';

class InitialIntro extends StatefulWidget {
  @override
  _InitialIntroState createState() => _InitialIntroState();
}

class _InitialIntroState extends State<InitialIntro> {
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
          InitialIntroCard(
              cardText: 'ezMemPass helps you make strong, easy to remember passwords! Card 1'),
          InitialIntroCard(cardText: 'Card 2'),
          InitialIntroCard(cardText: 'Card 3'),
        ],
      );
    });

    final Widget dotsIndicator = new DotsIndicator(
      dotsCount: 3,
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
//        slider,
        Container(
          padding: const EdgeInsets.all(10),
          child: dotsIndicator,
        ),
        RaisedButton(
            onPressed: () {
              Navigator.push(context, MaterialPageRoute(builder: (context) => MyApp()));
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
