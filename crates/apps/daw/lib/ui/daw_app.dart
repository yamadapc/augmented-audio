import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/audio_io_state.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_daw_mock_ui/state/ui_state.dart';
import 'package:mobx/mobx.dart';

import 'main_content_layout.dart';

var uiState = UIState();

class DawApp extends StatelessWidget {
  const DawApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    var project = Project();
    var audioIOState = AudioIOState(
        inputDevices: ObservableList.of([
          AudioDevice(title: "Default input"),
          AudioDevice(title: "AudioFuse Studio"),
        ]),
        outputDevices: ObservableList.of([
          AudioDevice(title: "Default output"),
          AudioDevice(title: "AudioFuse Studio"),
        ]),
        availableInputs: ObservableList.of([
          AudioInput("none", "No input"),
          AudioInput("1", "Input 1"),
          AudioInput("2", "Input 2"),
        ]));
    var tracks = [
      Track(
          id: "1",
          title: "Track 1",
          clips: ObservableList.of([Clip(title: "Clip 1")]),
          parent: project.tracksList),
      Track(id: "2", title: "Track 2", parent: project.tracksList),
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
          child: MainContentLayout(
              title: 'DAW', project: project, uiState: uiState)),
    );
  }
}
