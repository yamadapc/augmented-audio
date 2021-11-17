import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';

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
          primarySwatch: Colors.purple,
          textTheme: const TextTheme(
            bodyText2: TextStyle(
              fontSize: 12,
            ),
          )),
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
    return Scaffold(
        backgroundColor: const Color.fromRGBO(35, 35, 38, 1.0),
        body: Column(children: [
          const Header(),
          Expanded(
              child: Row(
            children: const [
              Sidebar(),
              Expanded(child: TracksView()),
            ],
          )),
          const BottomPanel(),
        ]));
  }
}

class Sidebar extends StatelessWidget {
  const Sidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      child: Container(
          decoration:
              const BoxDecoration(color: Color.fromRGBO(50, 50, 50, 1.0)),
          child: null),
    );
  }
}

class TracksView extends StatefulWidget {
  const TracksView({Key? key}) : super(key: key);

  @override
  State<TracksView> createState() => _TracksViewState();
}

class _TracksViewState extends State<TracksView> {
  var tracks = [
    Track("1", "Track 1"),
    Track("2", "Track 2"),
    Track("3", "Track 3"),
    Track("4", "Track 4"),
  ];

  @override
  Widget build(BuildContext context) {
    return ReorderableListView(
      onReorder: (sourceIndex, targetIndex) {
        setState(() {
          var elem = tracks[sourceIndex];
          tracks.removeAt(sourceIndex);
          var targetPrime = Math.max(
              sourceIndex < targetIndex ? targetIndex - 1 : targetIndex, 0);
          tracks.insert(targetPrime, elem);
        });
      },
      physics:
          const BouncingScrollPhysics(parent: AlwaysScrollableScrollPhysics()),
      scrollDirection: Axis.horizontal,
      children: List.generate(tracks.length, (trackIndex) {
        var track = tracks[trackIndex];
        return JamTrackView(
            key: Key(track.id), title: track.title, index: trackIndex);
      }).toList(),
      buildDefaultDragHandles: false,
    );
  }
}

class BottomPanel extends StatelessWidget {
  const BottomPanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return DefaultTextStyle(
      style: textStyle,
      child: Container(
        decoration: BoxDecoration(boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.4),
            spreadRadius: 1.0,
            blurRadius: 5.0,
          )
        ], border: Border.all(color: const Color.fromRGBO(65, 65, 65, 1.0))),
        height: 200,
        child: Row(children: const [Text("Plugin 1")]),
      ),
    );
  }
}

class Header extends StatelessWidget {
  const Header({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return DefaultTextStyle.merge(
      style: textStyle,
      child: Container(
          height: 30,
          width: double.infinity,
          padding: const EdgeInsets.all(4.0),
          decoration: BoxDecoration(boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.4),
              spreadRadius: 1.0,
              blurRadius: 5.0,
            )
          ], border: Border.all(color: Colors.black)),
          child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [Text("DAW")])),
    );
  }
}

class Track {
  final String id;
  final String title;

  Track(this.id, this.title);
}

class JamTrackView extends StatelessWidget {
  final String title;
  final int index;

  const JamTrackView({
    Key? key,
    required this.title,
    required this.index,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRect(
      child: Container(
        decoration: const BoxDecoration(
            color: Color.fromRGBO(79, 79, 79, 1.0),
            border: Border(
              left: BorderSide(color: Color.fromRGBO(65, 65, 65, 0.0)),
              right: BorderSide(color: Color.fromRGBO(65, 65, 65, 1.0)),
            )),
        child: SizedBox(
            width: 120,
            child: Column(
              mainAxisAlignment: MainAxisAlignment.start,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: <Widget>[
                // Track heading
                ReorderableDragStartListener(
                  index: index,
                  child: TrackTitle(title: title),
                ),
                Expanded(
                    child: Column(children: const [
                  ClipView(title: "Clip 1"),
                  ClipView(title: "Clip 2"),
                  ClipView(title: "Clip 3"),
                  ClipView(title: "Clip 4"),
                  ClipSlot(),
                  ClipSlot(),
                  ClipSlot(),
                  ClipSlot(),
                  ClipSlot(),
                  ClipSlot(),
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
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return SizedBox(
      width: double.infinity,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
            color: Color.fromRGBO(60, 60, 60, 1.0),
            border:
                Border(top: BorderSide(color: Color.fromRGBO(90, 90, 90, 1.0))),
          ),
          child: DefaultTextStyle.merge(
            style: textStyle,
            child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Column(children: const [
                          Knob(),
                          Knob(),
                        ]),
                      ),
                      const SizedBox(width: 30, child: VolumeMeter()),
                    ],
                  ),
                  DropdownButton(
                      dropdownColor: const Color.fromRGBO(30, 30, 30, 1.0),
                      style: textStyle,
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
                ]),
          )),
    );
  }

  void onChanged(Object? value) {}
}

class Knob extends StatelessWidget {
  const Knob({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        height: 50,
        width: 50,
        decoration: const BoxDecoration(
          color: Colors.black,
          shape: BoxShape.circle,
        ),
        child: null);
  }
}

class VolumeMeter extends StatelessWidget {
  const VolumeMeter({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
        height: 150,
        child: Container(
          decoration: BoxDecoration(
              border: Border.all(color: const Color.fromRGBO(90, 90, 90, 1.0))),
          child: SceneBuilderWidget(
              builder: () => SceneController(
                    config: SceneConfig.autoRender,
                    back: VolumeMeterScene(),
                  )),
        ));
  }
}

class VolumeMeterScene extends GSprite {
  late GShape rectangleLeft;
  late GShape rectangleRight;
  var tick = 0.0;

  @override
  void addedToStage() {
    var backgroundLeft = GShape();
    backgroundLeft.graphics
      ..beginFill(const Color.fromRGBO(54, 54, 54, 1.0))
      ..drawRect(0, 0, 11.5, (stage?.stageHeight ?? 0))
      ..endFill();
    addChild(backgroundLeft);
    var backgroundRight = GShape();
    backgroundRight.graphics
      ..beginFill(const Color.fromRGBO(54, 54, 54, 1.0))
      ..drawRect(15.0, 0, 11.5, (stage?.stageHeight ?? 0))
      ..endFill();
    addChild(backgroundRight);

    var volumeWidth = 10.0;
    var volumeHeight = 40.0;
    rectangleLeft = GShape();
    rectangleLeft.graphics.lineStyle(1.0, Colors.green)
      ..beginFill(Colors.green)
      ..drawRect(2.5, (stage?.stageHeight ?? 0) - volumeHeight, volumeWidth,
          volumeHeight)
      ..endFill();
    addChild(rectangleLeft);

    rectangleRight = GShape();
    rectangleRight.graphics.lineStyle(1.0, Colors.green)
      ..beginFill(Colors.green)
      ..drawRect(volumeWidth + 5.0, (stage?.stageHeight ?? 0) - volumeHeight,
          volumeWidth, volumeHeight)
      ..endFill();
    addChild(rectangleRight);
  }

  @override
  void update(double delta) {
    super.update(delta);

    tick += delta;

    var volumeHeight = 40.0;
    var height = volumeHeight * (1 + Math.sin(tick * 4.0));
    rectangleLeft.height = height;
    rectangleLeft.y = stage?.stageHeight;
    rectangleLeft.pivotY = stage?.stageHeight ?? 0;

    height = volumeHeight * (1 + Math.cos(tick * 4.0));
    rectangleRight.height = height;
    rectangleRight.y = stage?.stageHeight;
    rectangleRight.pivotY = stage?.stageHeight ?? 0;
    // print(rectangle.y);
  }
}

class ClipSlot extends StatelessWidget {
  const ClipSlot({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 35,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
              color: Color.fromRGBO(50, 50, 50, 1),
              border: Border(bottom: BorderSide(color: Colors.black))),
          child: null),
    );
  }
}

class ClipView extends StatelessWidget {
  final String title;
  const ClipView({Key? key, required this.title}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 35,
      child: Container(
          decoration: const BoxDecoration(
              color: Color.fromRGBO(50, 50, 50, 1),
              border: Border(bottom: BorderSide(color: Colors.black))),
          child: Container(
              margin: const EdgeInsets.all(1.0),
              padding: const EdgeInsets.only(left: 6.0, right: 6.0),
              decoration: const BoxDecoration(
                  color: Color.fromRGBO(89, 199, 228, 1),
                  border: Border(bottom: BorderSide(color: Colors.black))),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Expanded(child: Text(title)),
                  SizedBox(
                    height: 20,
                    width: 20,
                    child: IconButton(
                        padding: const EdgeInsets.all(0),
                        onPressed: () {},
                        icon: const Icon(
                          Icons.stop,
                          size: 20,
                        )),
                  ),
                ],
              ))),
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
