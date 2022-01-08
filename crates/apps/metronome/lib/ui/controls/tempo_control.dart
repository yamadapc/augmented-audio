import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

import '../../bridge_generated.dart';
import '../constants.dart';

class TempoControl extends StatelessWidget {
  const TempoControl({
    Key? key,
    required this.tempo,
    required this.metronome,
  }) : super(key: key);

  final Observable<double> tempo;
  final Metronome metronome;

  @override
  Widget build(BuildContext context) {
    return Observer(
        builder: (_) => Column(children: [
              Text("tempo", textScaleFactor: .8, style: labelTextStyle),
              Text(tempo.value.toStringAsFixed(0), textScaleFactor: 5.0),
              SizedBox(
                width: double.infinity,
                child: CupertinoSlider(
                    value: tempo.value,
                    onChanged: (value) {
                      metronome.setTempo(value: value);
                      runInAction(() {
                        tempo.value = value;
                      });
                    }, // onTempoChanged,
                    min: 30,
                    max: 250),
              )
            ]));
  }
}
