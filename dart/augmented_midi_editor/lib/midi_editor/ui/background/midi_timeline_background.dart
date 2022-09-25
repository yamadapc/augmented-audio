import 'package:flutter/material.dart';

class MIDITimelineBackground extends StatelessWidget {
  const MIDITimelineBackground({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(children: [
      const SizedBox(width: 110),
      Expanded(
        child: LayoutBuilder(
          builder: (_, boxConstraints) {
            var bars = [0, 1, 2, 3];
            var barViews =
                bars.map((i) => BarBackgroundView(index: i)).toList();

            var breakLines = [];
            var linesPerBar = 8;
            var totalLines = linesPerBar * bars.length;
            var lineDistance = boxConstraints.maxWidth / totalLines;

            for (int i = 0; i < totalLines; i++) {
              breakLines.add(Positioned(
                  left: i * lineDistance,
                  top: 0,
                  height: boxConstraints.maxHeight,
                  child: VerticalDivider(
                    width: 1,
                    color: Colors.black.withOpacity(0.15),
                  )));
            }

            return Stack(
              children: [
                Row(mainAxisSize: MainAxisSize.max, children: barViews),
                ...breakLines,
              ],
            );
          },
        ),
      ),
    ]);
  }
}

class BarBackgroundView extends StatelessWidget {
  final int index;

  const BarBackgroundView({
    Key? key,
    required this.index,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
          height: double.infinity,
          decoration: BoxDecoration(
              color: index % 2 == 0
                  ? const Color.fromRGBO(100, 100, 100, 0.3)
                  : const Color.fromRGBO(70, 70, 70, 0.3)),
          child: null),
    );
  }
}
