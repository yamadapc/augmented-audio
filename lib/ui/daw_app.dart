import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/audio_io_state.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:mobx/mobx.dart';

import 'main_content_layout.dart';

class DawApp extends StatelessWidget {
  const DawApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    var project = Project();
    var audioIOState = AudioIOState(ObservableList.of([
      AudioInput("none", "No input"),
      AudioInput("1", "Input 1"),
      AudioInput("2", "Input 2"),
    ]));
    var tracks = [
      Track(
          id: "1",
          title: "Track 1",
          clips: ObservableList.of([Clip(title: "Clip 1")])),
      Track(id: "2", title: "Track 2"),
      Track(id: "3", title: "Track 3"),
      Track(id: "4", title: "Track 4"),
      Track(id: "5", title: "Track 5"),
      Track(id: "6", title: "Track 6"),
      Track(id: "7", title: "Track 7"),
      Track(id: "8", title: "Track 8"),
      Track(id: "9", title: "Track 9"),
      Track(id: "10", title: "Track 10"),
    ];
    project.tracksList.tracks.addAll(tracks);

    return MaterialApp(
      title: 'DAW Demo',
      theme: ThemeData(
          primarySwatch: Colors.blue,
          textTheme: const TextTheme(
            bodyText2: TextStyle(
              fontSize: 12,
            ),
          )),
      home: AudioIOStateProvider(
          audioIOState: audioIOState,
          child: MainContentLayout(title: 'DAW', project: project)),
    );
  }
}
