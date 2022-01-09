import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';

import '../constants.dart';

class TempoControl extends StatelessWidget {
  const TempoControl({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  final MetronomeStateController stateController;

  @override
  Widget build(BuildContext context) {
    var model = stateController.model;
    return Observer(
        builder: (_) =>
            Column(children: [
              Text("tempo", textScaleFactor: .8, style: labelTextStyle),
              Text(model.tempo.toStringAsFixed(0), textScaleFactor: 5.0),
              SizedBox(
                width: double.infinity,
                child: CupertinoSlider(
                    value: model.tempo,
                    onChanged: (value) {
                      stateController.setTempo(value);
                    }, // onTempoChanged,
                    min: 30,
                    max: 250),
              )
            ]));
  }
}
