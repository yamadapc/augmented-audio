// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:firebase_core/firebase_core.dart';
// import 'package:flutter/cupertino.dart';
// import 'package:flutter/material.dart';
// import 'package:metronome/firebase_options.dart';
// import 'package:metronome/modules/analytics/analytics_impl.dart';
// import 'package:metronome/ui/app.dart';
//
// void main() async {
//   WidgetsFlutterBinding.ensureInitialized();
//   await Firebase.initializeApp(
//     options: DefaultFirebaseOptions.currentPlatform,
//   );
//
//   runApp(App(analytics: AnalyticsImpl()));
// }
//

import 'package:flutter/material.dart';
import 'package:metronome/src/rust/api/simple.dart';
import 'package:metronome/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
