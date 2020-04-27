import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_app/models/preferences_model.dart';
import 'package:flutter_app/widgets/privacy_policy_widget.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:provider/provider.dart';

class SettingsWidget extends StatelessWidget {
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
            title: Text("Settings"),
            leading: new IconButton(
              icon: backIcon,
              onPressed: () {
                Navigator.of(context).pop();
              },
            )),
        body: SettingsListView());
  }
}

const ABOUT_TEXT = """## About EzMemPass

**Version**: 1.0.0 build 6
**Authors**: Asim Ihsan and Jack Vaught

Please send comments, questions, and feedback to:

### **ezmempass@gmail.com**
""";

class SettingsListView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final Brightness brightnessValue = MediaQuery.of(context).platformBrightness;
    final bool isDark = brightnessValue == Brightness.dark;

    return SafeArea(
        child: Container(
      padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 8.0),
      child: ListView(
        physics: ClampingScrollPhysics(),
        children: <Widget>[
          Card(
            child: ListTile(
              title: Text('Reset settings'),
              trailing: Icon(Icons.keyboard_arrow_right),
              onTap: () async {
                final PreferencesModel preferencesModel =
                    Provider.of<PreferencesModel>(context, listen: false);
                await preferencesModel.clear();
                Scaffold.of(context)
                    .showSnackBar(SnackBar(content: Text("Successfully reset settings.")));
              },
            ),
          ),
          Card(
            child: ListTile(
              title: Text('Privacy policy'),
              trailing: Icon(Icons.keyboard_arrow_right),
              onTap: () async {
                await Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => PrivacyPolicyWidget()),
                );
              },
            ),
          ),
          Card(
            child: Container(
              padding: EdgeInsets.symmetric(vertical: 8.0, horizontal: 8.0),
              child: MarkdownBody(
                  data: ABOUT_TEXT,
                  styleSheet: MarkdownStyleSheet.fromTheme(Theme.of(context)).copyWith(
                    blockquoteDecoration: BoxDecoration(
                      color: isDark ? Colors.blue.withOpacity(0.4) : Colors.blue.shade50,
                      borderRadius: BorderRadius.circular(2.0),
                    ),
                  )),
            ),
          ),
        ],
      ),
    ));
  }
}
