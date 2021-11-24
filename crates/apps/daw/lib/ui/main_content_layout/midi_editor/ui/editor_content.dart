import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
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

class MIDIEditorContentView extends StatefulWidget {
  final MIDIClipModel model;

  const MIDIEditorContentView({
    Key? key,
    required this.model,
  }) : super(key: key);

  @override
  State<MIDIEditorContentView> createState() => _MIDIEditorContentViewState();
}

class _MIDIEditorContentViewState extends State<MIDIEditorContentView> {
  final FocusNode focusNode = FocusNode();

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (_, boxConstraints) {
        var rowPositions = notes
            .asMap()
            .map((key, value) => MapEntry(value.getSymbol(), key * 21));

        return Observer(
          builder: (context) => Focus(
            focusNode: focusNode,
            onKey: onKey,
            onFocusChange: onFocusChange,
            child: buildContent(context, boxConstraints, rowPositions),
          ),
        );
      },
    );
  }

  Stack buildContent(BuildContext context, BoxConstraints boxConstraints,
      Map<String, int> rowPositions) {
    return Stack(
      children: [
        RepaintBoundary(
          child: Column(
              children: notes
                  .map((note) => MIDINoteLane(note: note, model: widget.model))
                  .toList()),
        ),
        ...widget.model.midiNotes
            .map((note) => MIDINoteView(
                  note: note,
                  rowPositions: rowPositions,
                  isSelected: widget.model.selectedNotes.contains(note),
                  parentWidth: boxConstraints.maxWidth - 110,
                  onTap: () => onTap(context, note),
                  onDragUpdate: (details) =>
                      onDragUpdate(context, note, details),
                ))
            .toList()
      ],
    );
  }

  void onFocusChange(hasFocus) {
    if (!hasFocus) {
      widget.model.unselectNotes();
    }
  }

  KeyEventResult onKey(FocusNode node, RawKeyEvent value) {
    var deleteKeys = {
      LogicalKeyboardKey.delete.keyId,
      LogicalKeyboardKey.backspace.keyId,
    };

    if (value is RawKeyUpEvent && deleteKeys.contains(value.logicalKey.keyId)) {
      onDelete();
      return KeyEventResult.handled;
    } else if (value is RawKeyDownEvent &&
        deleteKeys.contains(value.logicalKey.keyId)) {
      return KeyEventResult.handled;
    }

    return KeyEventResult.ignored;
  }

  onDragUpdate(
      BuildContext context, MIDINoteModel note, DragUpdateDetails details) {
    var renderBox = context.findRenderObject() as RenderBox;
    var localPosition = renderBox.globalToLocal(details.globalPosition);
    var index = (localPosition.dy / 21);
    var newNote = notes[index.toInt()];
    note.note = newNote;
  }

  onTap(BuildContext context, MIDINoteModel note) {
    widget.model.setSelectedNote(note);
    FocusScope.of(context).requestFocus(focusNode);
  }

  onDelete() {
    for (var note in widget.model.selectedNotes.toList()) {
      widget.model.removeNote(note);
    }
  }
}
