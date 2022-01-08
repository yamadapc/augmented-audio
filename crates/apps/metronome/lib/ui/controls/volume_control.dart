import 'package:flutter/cupertino.dart';
import 'package:mobx/mobx.dart';

import '../../bridge_generated.dart';
import '../constants.dart';

class VolumeControl extends StatelessWidget {
  const VolumeControl({
    Key? key,
    required this.volume,
    required this.metronome,
  }) : super(key: key);

  final Observable<double> volume;
  final Metronome metronome;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Text("volume", style: labelTextStyle, textScaleFactor: 0.8),
        SizedBox(
            width: double.infinity,
            child: CupertinoSlider(
                min: 0,
                max: 1.0,
                value: volume.value,
                onChanged: (value) {
                  metronome.setVolume(value: value);
                  runInAction(() {
                    volume.value = value;
                  });
                })),
      ],
    );
  }
}
