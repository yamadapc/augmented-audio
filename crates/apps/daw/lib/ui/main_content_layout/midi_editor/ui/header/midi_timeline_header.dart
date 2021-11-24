import 'package:flutter/material.dart';

class MIDITimelineHeader extends StatelessWidget {
  const MIDITimelineHeader({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      child: Row(children: [
        const SizedBox(height: 20, width: 110),
        Expanded(
          child: Row(mainAxisSize: MainAxisSize.max, children: [
            Expanded(
              child: Container(
                  height: 20,
                  decoration:
                      const BoxDecoration(color: Color.fromRGBO(70, 70, 70, 1)),
                  child: null),
            ),
            Expanded(
              child: Container(
                  height: 20,
                  decoration: const BoxDecoration(
                      color: Color.fromRGBO(100, 100, 100, 1)),
                  child: null),
            ),
            Expanded(
              child: Container(
                  height: 20,
                  decoration:
                      const BoxDecoration(color: Color.fromRGBO(70, 70, 70, 1)),
                  child: null),
            ),
            Expanded(
              child: Container(
                  height: 20,
                  decoration: const BoxDecoration(
                      color: Color.fromRGBO(100, 100, 100, 1)),
                  child: null),
            ),
          ]),
        ),
      ]),
    );
  }
}
