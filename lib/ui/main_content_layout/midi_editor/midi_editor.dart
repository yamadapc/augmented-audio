import 'package:flutter/material.dart';

class MIDIEditorView extends StatelessWidget {
  const MIDIEditorView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    List<String> notes = [
      "C3",
      "C#3",
      "D3",
      "D#3",
      "E3",
      "F3",
      "F#3",
      "G3",
      "G#3",
      "A3",
      "A#3",
      "B3",
      "C4",
      "C#4",
      "D4",
      "D#4",
      "E4",
      "F4",
      "F#4",
      "G4",
      "G#4",
      "A4",
      "A#4",
      "B4",
    ].reversed.toList();

    return DefaultTextStyle(
      style: const TextStyle(color: Colors.black),
      child: Container(
        decoration:
            const BoxDecoration(color: Color.fromRGBO(120, 120, 120, 1)),
        child: SingleChildScrollView(
          child: GestureDetector(
            onTapUp: onTapUp,
            child: Column(
                children:
                    notes.map((note) => MIDINoteLane(note: note)).toList()),
          ),
        ),
      ),
    );
  }

  void onTapUp(TapUpDetails details) {
    print(details.localPosition);
  }
}

class MIDINoteLane extends StatelessWidget {
  final String note;

  const MIDINoteLane({Key? key, required this.note}) : super(key: key);

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
        SizedBox(width: 50, child: Text(note)),
        PianoKeyView(isSharp: isSharp()),
        Expanded(
            child: Container(
                decoration: const BoxDecoration(
                  color: Color.fromRGBO(200, 200, 200, 1),
                ),
                child: Row(children: const [])))
      ]),
    );
  }

  bool isSharp() {
    return note.contains("#");
  }
}

class PianoKeyView extends StatelessWidget {
  final bool isSharp;

  const PianoKeyView({Key? key, required this.isSharp}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.only(left: 8),
        width: 50,
        height: 20,
        decoration: BoxDecoration(
          color: isSharp ? Colors.black : Colors.white,
        ),
        child: null);
  }
}
