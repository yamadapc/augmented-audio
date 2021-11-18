import 'package:flutter/material.dart';

class TrackTitle extends StatelessWidget {
  const TrackTitle({
    Key? key,
    required this.title,
    required this.index,
  }) : super(key: key);

  final String title;
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
                  title,
                ),
              ),
            )));
  }
}
