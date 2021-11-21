import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';

class TrackTitle extends StatelessWidget {
  const TrackTitle({
    Key? key,
    required this.track,
    required this.index,
  }) : super(key: key);

  final Track track;
  final int index;

  @override
  Widget build(BuildContext context) {
    return Scrollable(
        axisDirection: AxisDirection.down,
        viewportBuilder: (context, offset) => ReorderableDragStartListener(
            index: index,
            child: SizedBox(
              width: double.infinity,
              child: Container(
                padding: const EdgeInsets.all(8.0),
                decoration: const BoxDecoration(
                    color: Colors.white38,
                    border: Border(bottom: BorderSide(color: Colors.black))),
                child: Text(
                  track.title,
                ),
              ),
            )));
  }
}
