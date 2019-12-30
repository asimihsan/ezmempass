import 'package:carousel_slider/carousel_slider.dart';
import 'package:dots_indicator/dots_indicator.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_app/main.dart';

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
        height: MediaQuery.of(context).size.height * 0.9,
        viewportFraction: 1.0,
        initialPage: 0,
        autoPlay: false,
        scrollDirection: Axis.horizontal,
        enableInfiniteScroll: false,
        onPageChanged: (index) {
          setState(() {
            _current = index;
          });
        },
        items: <Widget>[
          Container(
              padding: const EdgeInsets.all(64),
              width: MediaQuery.of(context).size.width,
              margin: EdgeInsets.all(40.0),
              decoration: BoxDecoration(color: Theme.of(context).canvasColor),
              child: Text(
                'EzMemPass helps you make strong, easy to remember passwords! Page 1',
                style: Theme.of(context)
                    .textTheme
                    .headline
                    .copyWith(fontFamily: 'RecursiveSansLinear'),
              )),
          Container(
              padding: const EdgeInsets.all(64),
              width: MediaQuery.of(context).size.width,
              margin: EdgeInsets.all(40.0),
              decoration: BoxDecoration(color: Theme.of(context).canvasColor),
              child: Text(
                'EzMemPass helps you make strong, easy to remember passwords! Page 2',
                style: Theme.of(context)
                    .textTheme
                    .headline
                    .copyWith(fontFamily: 'RecursiveSansLinear'),
              )),
          Container(
              padding: const EdgeInsets.all(64),
              width: MediaQuery.of(context).size.width,
              margin: EdgeInsets.all(40.0),
              decoration: BoxDecoration(color: Theme.of(context).canvasColor),
              child: Text(
                'EzMemPass helps you make strong, easy to remember passwords! Page 3',
                style: Theme.of(context)
                    .textTheme
                    .headline
                    .copyWith(fontFamily: 'RecursiveSansLinear'),
              )),
        ],
      );
    });
    final Widget dots_indicator = new DotsIndicator(dotsCount: 3, position: _current.toDouble());
    final Widget sliderContainer = Builder(builder: (BuildContext context) {
      return Column(children: <Widget>[
        slider,
        dots_indicator,
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
        home: sliderContainer);
  }
}
