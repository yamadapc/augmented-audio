import 'package:flutter/material.dart';

import 'main_content_layout.dart';

class DawApp extends StatelessWidget {
  const DawApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'DAW Demo',
      theme: ThemeData(
          primarySwatch: Colors.purple,
          textTheme: const TextTheme(
            bodyText2: TextStyle(
              fontSize: 12,
            ),
          )),
      home: const MainContentLayout(title: 'DAW'),
    );
  }
}
