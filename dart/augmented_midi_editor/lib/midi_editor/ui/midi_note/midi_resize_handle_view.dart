import 'package:flutter/material.dart';
import 'package:mobx/mobx.dart';

import '../../midi_model.dart';

class MIDIResizeHandleView extends StatelessWidget {
  final MIDINoteModel note;
  final double width;
  final bool isLeftHandle;

  const MIDIResizeHandleView(
      {Key? key,
      required this.note,
      required this.width,
      required this.isLeftHandle})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onPanUpdate: onPanUpdate,
      child: MouseRegion(
        cursor: SystemMouseCursors.resizeLeftRight,
        child: Container(
            height: 20,
            width: 2,
            decoration: const BoxDecoration(color: Colors.red)),
      ),
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    var dx = details.delta.dx / width;
    runInAction(() {
      if (isLeftHandle) {
        note.time += dx;
        note.duration -= dx;
      } else {
        note.duration += dx;
      }
    });
  }
}
