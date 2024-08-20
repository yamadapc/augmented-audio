// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// // The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.
//
// // // The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.
// //
// // // import 'dart:ffi';
// // //
// // // import 'package:augmented_audio_wave_example/bridge_generated.dart';
// // // import 'package:flutter/material.dart';
// // //
// // // void main() {
// // //   runApp(const MyApp());
// // // }
// // //
// // // class MyApp extends StatefulWidget {
// // //   const MyApp({super.key});
// // //
// // //   @override
// // //   State<MyApp> createState() => _MyAppState();
// // // }
// // //
// // // class _MyAppState extends State<MyApp> {
// // //   var nativeApi = AugmentedAudioWaveExampleImpl(DynamicLibrary.open("augmented_audio_wave_example.dylib"));
// // //   var value = 0;
// // //
// // //   @override
// // //   void initState() {
// // //     super.initState();
// // //
// // //     nativeApi.add(left: 10, right: 20).then((value) => {
// // //       setState(() {
// // //         this.value = value;
// // //       })
// // //     });
// // //   }
// // //
// // //   @override
// // //   Widget build(BuildContext context) {
// // //     return MaterialApp(
// // //       title: 'Flutter Demo',
// // //       theme: ThemeData(
// // //         primarySwatch: Colors.blue,
// // //       ),
// // //       home: Scaffold(body: Center(child: Text("add(10, 20) => $value"))),
// // //     );
// // //   }
// // // }
// // //
// //
// // import 'package:flutter/material.dart';
// // import 'package:augmented_audio_wave_example/src/rust/api/simple.dart';
// // import 'package:augmented_audio_wave_example/src/rust/frb_generated.dart';
// //
// // Future<void> main() async {
// //   await RustLib.init();
// //   runApp(const MyApp());
// // }
// //
// // class MyApp extends StatelessWidget {
// //   const MyApp({super.key});
// //
// //   @override
// //   Widget build(BuildContext context) {
// //     return MaterialApp(
// //       home: Scaffold(
// //         appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
// //         body: Center(
// //           child: Text(
// //               'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
// //         ),
// //       ),
// //     );
// //   }
// // }
// //
//
// import 'package:flutter/material.dart';
// import 'package:augmented_audio_wave_example/src/rust/api/simple.dart';
// import 'package:augmented_audio_wave_example/src/rust/frb_generated.dart';
//
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const MyApp());
// }
//
// class MyApp extends StatelessWidget {
//   const MyApp({super.key});
//
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
//         body: Center(
//           child: Text(
//               'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
//         ),
//       ),
//     );
//   }
// }
//

import 'package:flutter/material.dart';
import 'package:augmented_audio_wave_example/src/rust/api/simple.dart';
import 'package:augmented_audio_wave_example/src/rust/frb_generated.dart';

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
