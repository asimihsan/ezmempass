import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';

Future<String> loadHelpText(BuildContext context) async {
  return await DefaultAssetBundle.of(context).loadString('assets/help_en.md');
}

class HelpScreen extends StatelessWidget {
  HelpScreen({Key key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    Icon backIcon;
    if (Platform.isIOS) {
      backIcon = new Icon(CupertinoIcons.back);
    } else {
      backIcon = new Icon(Icons.arrow_back);
    }

    return Scaffold(
      appBar: AppBar(
        title: Text("Help"),
        leading: new IconButton(
          icon: backIcon,
          onPressed: () {
            Navigator.of(context).pop();
          },
        )
      ),
      body: SingleChildScrollView(
        physics: ClampingScrollPhysics(),
        child: Container(
          padding: const EdgeInsets.all(16),
          child: FutureBuilder(
            future: loadHelpText(context),
            builder: (context, snapshot) {
              return MarkdownBody(
                  data: snapshot.data ?? '');
            },
          ),
        ),
      )
    );
  }
}