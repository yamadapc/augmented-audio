import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/tracks_view/track_view/track_controls/knob.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

abstract class KnobFieldModel {
  double getValue();
  void setValue(double value);
}

class KnobField extends StatelessWidget {
  final String label;
  final KnobFieldModel model;

  const KnobField({Key? key, required this.label, required this.model})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IntrinsicWidth(
        child: IntrinsicHeight(
            child: Stack(children: [
      Observer(
          builder: (_) => Knob(
              value: model.getValue(),
              onChange: (value) => model.setValue(value))),
      Positioned(
          bottom: 0, left: 0, width: 50, child: Center(child: Text(label)))
    ])));
  }
}
