import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_markdown/flutter_markdown.dart';

Future<String> loadPrivacyPolicyText(BuildContext context) async {
  return await DefaultAssetBundle.of(context).loadString('assets/privacy_policy.md');
}

class PrivacyPolicyWidget extends StatelessWidget {
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
            title: Text("Privacy Policy"),
            leading: new IconButton(
              icon: backIcon,
              onPressed: () {
                Navigator.of(context).pop();
              },
            )),
        body: SingleChildScrollView(
          physics: ClampingScrollPhysics(),
          child: Container(
            padding: const EdgeInsets.all(16),
            child: FutureBuilder(
              future: loadPrivacyPolicyText(context),
              builder: (context, snapshot) {
                final Brightness brightnessValue = MediaQuery.of(context).platformBrightness;
                bool isDark = brightnessValue == Brightness.dark;
                return MarkdownBody(
                    data: snapshot.data ?? '',
                    styleSheet: MarkdownStyleSheet.fromTheme(Theme.of(context)).copyWith(
                      blockquoteDecoration: BoxDecoration(
                        color: isDark ? Colors.blue.withOpacity(0.4) : Colors.blue.shade50,
                        borderRadius: BorderRadius.circular(2.0),
                      ),
                    ));
              },
            ),
          ),
        ));
  }
}
