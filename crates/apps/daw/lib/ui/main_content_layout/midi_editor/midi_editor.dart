import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/services/state_sync.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'midi_model.dart';

List<Note> notes = [
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
].reversed.map((note) => Note.ofSymbol(note)).toList();

class MIDIEditorView extends StatelessWidget {
  final MIDIClipModel model;

  const MIDIEditorView({Key? key, required this.model}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DefaultTextStyle(
      style: DefaultTextStyle.of(context)
          .style
          .merge(const TextStyle(color: Colors.black)),
      child: Container(
        decoration:
            const BoxDecoration(color: Color.fromRGBO(120, 120, 120, 1)),
        child: Column(
          children: [
            const MIDITimelineHeader(),
            Expanded(
              child: Stack(
                children: [
                  const RepaintBoundary(child: MIDITimelineBackground()),
                  SingleChildScrollView(
                    child: MidiEditorContentView(model: model),
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

class MidiEditorContentView extends StatelessWidget {
  final MIDIClipModel model;

  const MidiEditorContentView({
    Key? key,
    required this.model,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (_, boxConstraints) {
        var rowPositions = notes.asMap().map((key, value) {
          return MapEntry(value.getSymbol(), key * 21);
        });

        return Observer(
          builder: (context) => Stack(
            children: [
              RepaintBoundary(
                child: Column(
                    children: notes
                        .map((note) => MIDINoteLane(note: note, model: model))
                        .toList()),
              ),
              ...model.midiNotes
                  .map((note) => MIDINoteView(
                        note: note,
                        rowPositions: rowPositions,
                        isSelected: model.selectedNotes.contains(note),
                        parentWidth: boxConstraints.maxWidth - 110,
                        onTap: () => onTap(note),
                        onDragUpdate: (details) =>
                            onDragUpdate(context, note, details),
                      ))
                  .toList()
            ],
          ),
        );
      },
    );
  }

  onDragUpdate(
      BuildContext context, MIDINoteModel note, DragUpdateDetails details) {
    var renderBox = context.findRenderObject() as RenderBox;
    var localPosition = renderBox.globalToLocal(details.globalPosition);
    var index = (localPosition.dy / 21);
    var newNote = notes[index.toInt()];
    note.note = newNote;
  }

  onTap(MIDINoteModel note) {
    model.setSelectedNote(note);
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

class MIDINoteView extends StatelessWidget {
  final MIDINoteModel note;
  final Map<String, int> rowPositions;
  final double parentWidth;
  final void Function(DragUpdateDetails) onDragUpdate;
  final void Function() onTap;
  final bool isSelected;

  const MIDINoteView({
    Key? key,
    required this.note,
    required this.isSelected,
    required this.rowPositions,
    required this.parentWidth,
    required this.onDragUpdate,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        var notePosition = 110 + note.time * parentWidth;
        var noteWidth = note.duration * parentWidth;
        var height = 20.0;
        var noteTop = rowPositions[note.note.getSymbol()]!.toDouble();

        return Positioned(
          top: noteTop,
          left: notePosition,
          child: RepaintBoundary(
            child: SizedBox(
              width: noteWidth,
              height: height,
              child: Row(
                children: [
                  MIDIResizeHandleView(
                      note: note, width: parentWidth, isLeftHandle: true),
                  Expanded(
                    child: GestureDetector(
                      onTap: onTap,
                      onPanUpdate: onPanUpdate,
                      child: Container(
                        decoration: BoxDecoration(
                            color: Colors.blue
                                .withOpacity(isSelected ? 1.0 : 0.5)),
                        width: noteWidth,
                        height: height,
                        child: null,
                      ),
                    ),
                  ),
                  MIDIResizeHandleView(
                      note: note, width: parentWidth, isLeftHandle: false),
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    var dx = details.delta.dx / parentWidth;
    runInEntity(note, () {
      note.time += dx;
    });
    onDragUpdate(details);
  }
}

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
    runInEntity(note, () {
      if (isLeftHandle) {
        note.time += dx;
        note.duration -= dx;
      } else {
        note.duration += dx;
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
