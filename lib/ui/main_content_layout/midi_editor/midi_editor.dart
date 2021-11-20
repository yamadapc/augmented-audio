import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

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

class MIDIEditorView extends StatelessWidget {
  const MIDIEditorView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DefaultTextStyle(
      style: const TextStyle(color: Colors.black),
      child: Container(
        decoration:
            const BoxDecoration(color: Color.fromRGBO(120, 120, 120, 1)),
        child: Column(
          children: [
            const MIDITimelineHeader(),
            Expanded(
              child: Stack(
                children: [
                  const MIDITimelineBackground(),
                  SingleChildScrollView(
                    child: Column(
                        children: notes
                            .map((note) => MIDINoteLane(note: note))
                            .toList()),
                  )
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

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

class MIDILaneModel {
  final List<MIDINote> notes;

  MIDILaneModel(this.notes);
}

class MIDINote {
  final Observable<double> time;
  final Observable<double> duration = Observable((1 / 4) / 4);

  MIDINote(this.time);
}

class MIDINoteLane extends StatelessWidget {
  final String note;
  final Observable<MIDILaneModel> midiLaneModel = Observable(MIDILaneModel([]));

  MIDINoteLane({Key? key, required this.note}) : super(key: key);

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
          child: GestureDetector(
            onTapUp: (details) => onTapUp(context, details),
            child: Container(
              width: double.infinity,
              height: 20,
              decoration: const BoxDecoration(
                // Without painting, there's no gesture detection above
                color: Colors.transparent,
              ),
              child: LayoutBuilder(
                  builder: (_, boxConstraints) => Observer(
                      builder: (_) => Stack(
                          children: midiLaneModel.value.notes
                              .map((note) => MIDINoteView(
                                  note: note, boxConstraints: boxConstraints))
                              .toList()))),
            ),
          ),
        )
      ]),
    );
  }

  void onTapUp(BuildContext context, TapUpDetails details) {
    var width = context.size!.width - 110;
    var x = details.localPosition.dx / width;
    var note = MIDINote(Observable(x));
    runInAction(() {
      var model = MIDILaneModel(midiLaneModel.value.notes);
      model.notes.add(note);
      midiLaneModel.value = model;
    });
  }

  bool isSharp() {
    return note.contains("#");
  }
}

class MIDINoteView extends StatelessWidget {
  final MIDINote note;
  final BoxConstraints boxConstraints;

  const MIDINoteView(
      {Key? key, required this.note, required this.boxConstraints})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        var notePosition = note.time.value * boxConstraints.maxWidth;
        var noteWidth = note.duration.value * boxConstraints.maxWidth;
        var height = 20.0;
        return Positioned(
          top: 0,
          left: notePosition,
          child: SizedBox(
            width: noteWidth,
            height: height,
            child: Row(
              children: [
                MIDIResizeHandleView(
                    note: note,
                    boxConstraints: boxConstraints,
                    isLeftHandle: true),
                Expanded(
                  child: GestureDetector(
                    onPanUpdate: onPanUpdate,
                    child: Container(
                      decoration: const BoxDecoration(color: Colors.blue),
                      width: noteWidth,
                      height: height,
                      child: null,
                    ),
                  ),
                ),
                MIDIResizeHandleView(
                    note: note,
                    boxConstraints: boxConstraints,
                    isLeftHandle: false),
              ],
            ),
          ),
        );
      },
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    var dx = details.delta.dx / boxConstraints.maxWidth;
    runInAction(() {
      note.time.value += dx;
    });
  }
}

class MIDIResizeHandleView extends StatelessWidget {
  final MIDINote note;
  final BoxConstraints boxConstraints;
  final bool isLeftHandle;

  const MIDIResizeHandleView(
      {Key? key,
      required this.note,
      required this.boxConstraints,
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
    var dx = details.delta.dx / boxConstraints.maxWidth;
    runInAction(() {
      if (isLeftHandle) {
        note.time.value += dx;
        note.duration.value -= dx;
      } else {
        note.duration.value += dx;
      }
    });
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
