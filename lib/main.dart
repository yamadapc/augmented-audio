import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        // This is the theme of your application.
        //
        // Try running your application with "flutter run". You'll see the
        // application has a blue toolbar. Then, without quitting the app, try
        // changing the primarySwatch below to Colors.green and then invoke
        // "hot reload" (press "r" in the console where you ran "flutter run",
        // or simply save your changes to "hot reload" in a Flutter IDE).
        // Notice that the counter didn't reset back to zero; the application
        // is not restarted.
        primarySwatch: Colors.purple,
      ),
      home: const MyHomePage(title: 'DAW'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    return Scaffold(
        appBar: AppBar(
          // Here we take the value from the MyHomePage object that was created by
          // the App.build method, and use it to set our appbar title.
          title: Text(widget.title),
        ),
        body: ReorderableListView(
          onReorder: (sourceIndex, targetIndex) {},
          scrollDirection: Axis.horizontal,
          children: const [
            JamTrackView(key: Key("1"), title: "Track 1"),
            JamTrackView(key: Key("2"), title: "Track 2"),
            JamTrackView(key: Key("3"), title: "Track 3"),
            JamTrackView(key: Key("4"), title: "Track 4"),
          ],
        ));
  }
}

class JamTrackView extends StatelessWidget {
  final String title;

  const JamTrackView({
    Key? key,
    required this.title,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
          border: Border(
              left: BorderSide(color: Colors.black),
              right: BorderSide(color: Colors.black))),
      child: Container(
        margin: const EdgeInsets.only(bottom: 40.0),
        child: SizedBox(
            width: 200,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                // Track heading
                TrackTitle(title: title),
                Expanded(
                    child: Column(children: const [
                  ClipView(title: "Clip 1"),
                  ClipView(title: "Clip 2"),
                  ClipView(title: "Clip 3"),
                  ClipView(title: "Clip 4"),
                ])),
                const TrackControls()
                // Clips
              ],
            )),
      ),
    );
  }
}

class TrackControls extends StatelessWidget {
  const TrackControls({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
              color: Colors.red,
              border: Border(
                  top: BorderSide(color: Colors.black),
                  bottom: BorderSide(color: Colors.black))),
          child: Column(
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Text("Track controls"),
                VolumeMeter(),
                DropdownButton(
                    isExpanded: true,
                    value: "Input 1",
                    items: const [
                      DropdownMenuItem(
                          child: Text("Input 1"), value: "Input 1"),
                      DropdownMenuItem(
                          child: Text("Input 2"), value: "Input 2"),
                      DropdownMenuItem(
                          child: Text("Input 3"), value: "Input 3"),
                    ],
                    onChanged: onChanged)
              ])),
    );
  }

  void onChanged(Object? value) {}
}

class VolumeMeter extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return SizedBox(
        height: 120,
        child: Container(
          decoration: BoxDecoration(border: Border.all(color: Colors.black)),
          child: const Text("canvas here"),
        ));
  }
}

class ClipView extends StatelessWidget {
  final String title;
  const ClipView({Key? key, required this.title}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
              color: Color(0xFF9E99FF),
              border: Border(bottom: BorderSide(color: Colors.black))),
          child: Text(title)),
    );
  }
}

class TrackTitle extends StatelessWidget {
  const TrackTitle({
    Key? key,
    required this.title,
  }) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Container(
        padding: const EdgeInsets.all(8.0),
        decoration: const BoxDecoration(
            color: Colors.white38,
            border: Border(bottom: BorderSide(color: Colors.black))),
        child: Text(
          title,
        ),
      ),
    );
  }
}
