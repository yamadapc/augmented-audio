import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:mobx/mobx.dart';

import '../controls/bottom_row.dart';
import '../controls/tempo_control.dart';
import '../controls/volume_control.dart';
import '../visualisation/visualisation.dart';

class MainPageTab extends StatelessWidget {
  const MainPageTab({
    Key? key,
    required this.playhead,
    required this.tempo,
    required this.metronome,
    required this.volume,
    required this.isPlaying,
  }) : super(key: key);

  final Observable<double> playhead;
  final Observable<double> tempo;
  final Metronome metronome;
  final Observable<double> volume;
  final Observable<bool> isPlaying;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          children: [
            Visualisation(playhead: playhead),
            Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: <Widget>[
                    TempoControl(tempo: tempo, metronome: metronome),
                    VolumeControl(volume: volume, metronome: metronome),
                    const Spacer(),
                    BottomRow(metronome: metronome, isPlaying: isPlaying)
                  ],
                ),
              ),
            )
          ],
        ),
      ),
    );
  }
}
