import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/tracks_view/track_view/track_controls/knob.dart';

class KnobField extends StatelessWidget {
  final String label;
  const KnobField({Key? key, required this.label}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IntrinsicWidth(
        child: IntrinsicHeight(
            child: Stack(children: [
      const Knob(),
      Positioned(
          bottom: 0, left: 0, width: 50, child: Center(child: Text(label)))
    ])));
  }
}
