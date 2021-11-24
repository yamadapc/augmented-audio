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
        child: Row(mainAxisSize: MainAxisSize.max, children: [
          Expanded(
            child: Container(
                height: double.infinity,
                decoration:
                    const BoxDecoration(color: Color.fromRGBO(70, 70, 70, 0.3)),
                child: null),
          ),
          Expanded(
            child: Container(
                height: double.infinity,
                decoration: const BoxDecoration(
                    color: Color.fromRGBO(100, 100, 100, 0.3)),
                child: null),
          ),
          Expanded(
            child: Container(
                height: double.infinity,
                decoration:
                    const BoxDecoration(color: Color.fromRGBO(70, 70, 70, 0.3)),
                child: null),
          ),
          Expanded(
            child: Container(
                height: double.infinity,
                decoration: const BoxDecoration(
                    color: Color.fromRGBO(100, 100, 100, 0.3)),
                child: null),
          ),
        ]),
      ),
    ]);
  }
}
