import 'package:flutter/material.dart';

class MIDITimelineHeader extends StatelessWidget {
  const MIDITimelineHeader({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var bars = [0, 1, 2, 3];
    var barViews = bars.map((i) => BarHeaderView(index: i)).toList();

    return SizedBox(
      width: double.infinity,
      child: Row(children: [
        const SizedBox(height: 20, width: 110),
        Expanded(
          child: Row(mainAxisSize: MainAxisSize.max, children: barViews),
        ),
      ]),
    );
  }
}

class BarHeaderView extends StatelessWidget {
  final int index;

  const BarHeaderView({
    Key? key,
    required this.index,
  }) : super(key: key);

  String get name => "${index + 1}.";

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: Container(
          height: 20,
          padding: const EdgeInsets.all(2),
          decoration: BoxDecoration(
              color: index % 2 == 0
                  ? const Color.fromRGBO(100, 100, 100, 1)
                  : const Color.fromRGBO(70, 70, 70, 1)),
          child: Text(name)),
    );
  }
}
