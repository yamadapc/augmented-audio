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
    // var api = wire.initialize();
    // api.initializeLogger();
    AudioIOState audioIOState = setupAudioIOState();
    // api.initializeAudio();

    // var stateSync = StateSyncService.get(api);
    // stateSync.start();

    Project project = setupProjectState();

    return MaterialApp(
      title: 'DAW',
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

  Project setupProjectState() {
    var project = Project();
    var tracks = [
      Track(
          id: "1",
          title: "Track 1",
          clips: ObservableList.of([Clip(title: "Clip 1")]),
          parent: project.tracksList),
      Track(id: "2", title: "Track 2", parent: project.tracksList),
    ];
    project.tracksList.tracks.addAll(tracks);
    return project;
  }

  AudioIOState setupAudioIOState() {
    var audioIOState = AudioIOState();
    // audioIOState.availableInputs = ObservableList.of([
    //   AudioInput("none", "No input"),
    //   AudioInput("1", "Input 1"),
    // ]);
    // var audioIOService = AudioIOService(api, audioIOState);
    // audioIOService.syncDevices();
    return audioIOState;
  }
}
