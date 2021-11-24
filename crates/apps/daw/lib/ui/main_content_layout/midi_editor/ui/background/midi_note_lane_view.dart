import 'package:flutter/material.dart';

import '../../midi_model.dart';
import 'piano_key_view.dart';

class MIDINoteLane extends StatelessWidget {
  final Note note;
  final MIDIClipModel model;

  const MIDINoteLane({Key? key, required this.note, required this.model})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        border: Border(
          bottom: BorderSide(color: Color.fromRGBO(80, 80, 80, 1.0)),
        ),
      ),
      width: double.infinity,
      child: Row(children: [
        SizedBox(width: 50, child: Text(note.getSymbol())),
        PianoKeyView(isSharp: note.isSharp()),
        Expanded(
          child: GestureDetector(
            onTapUp: (details) => onTapUp(context, details),
            child: Container(
              width: double.infinity,
              height: 20,
              decoration: const BoxDecoration(
                // Without painting, there's no gesture detection above
                color: Colors.transparent,
              ),
              child: null,
            ),
          ),
        )
      ]),
    );
  }

  void onTapUp(BuildContext context, TapUpDetails details) {
    var width = context.size!.width - 110;
    var x = details.localPosition.dx / width;
    model.addEvent(time: x, note: note);
  }
}
