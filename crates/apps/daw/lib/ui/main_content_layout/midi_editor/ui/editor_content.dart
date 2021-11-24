import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../midi_model.dart';
import 'background/midi_note_lane_view.dart';
import 'midi_note/midi_note_view.dart';

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

class MIDIEditorContentView extends StatelessWidget {
  final MIDIClipModel model;

  const MIDIEditorContentView({
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
