import 'package:flutter/material.dart';

class InitialIntroCard extends StatelessWidget {
  final String cardText;

  InitialIntroCard({
    @required this.cardText,
  });

  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(40),
      margin: EdgeInsets.only(
        top: 50.0,
        left: 40.0,
        right: 40.0,
        bottom: 10.0,
      ),
      decoration: BoxDecoration(color: Theme.of(context).canvasColor),
      child: Align(
          alignment: Alignment.center,
          child: Text(
            cardText,
            textAlign: TextAlign.left,
            style:
                Theme.of(context).textTheme.headline6.copyWith(fontFamily: 'RecursiveSansLinear'),
          )),
    );
  }
}
