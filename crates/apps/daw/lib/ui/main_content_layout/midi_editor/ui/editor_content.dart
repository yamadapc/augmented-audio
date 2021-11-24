import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_editor_view_model.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/ui/selection_overlay/selection_overlay.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../midi_model.dart';
import 'background/midi_note_lane_view.dart';
import 'midi_note/midi_note_view.dart';

List<String> baseNotes = [
  "C",
  "C#",
  "D",
  "D#",
  "E",
  "F",
  "F#",
  "G",
  "G#",
  "A",
  "A#",
  "B",
];

List<int> octaves = [
  0,
  1,
  2,
  3,
  4,
  5,
  6,
  7,
  8,
  9,
];

List<Note> notes = octaves
    .map((octave) =>
        baseNotes.map((noteLetter) => "$noteLetter$octave").toList())
    .fold(List<String>.empty(growable: true),
        (List<String> previousValue, List<String> element) {
      previousValue.addAll(element);
      return previousValue;
    })
    .reversed
    .map((note) => Note.ofSymbol(note))
    .toList();

class MIDIEditorContentView extends StatefulWidget {
  final MIDIEditorViewModel viewModel;
  final MIDIClipModel model;

  const MIDIEditorContentView({
    Key? key,
    required this.viewModel,
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
      builder: (_, boxConstraints) => Observer(
        builder: (context) {
          var rowPositions = notes.asMap().map((key, value) => MapEntry(
              value.getSymbol(), key * (widget.viewModel.noteHeight + 1)));

          return Focus(
            focusNode: focusNode,
            onKey: onKey,
            onFocusChange: onFocusChange,
            child: buildContent(context, boxConstraints, rowPositions),
          );
        },
      ),
    );
  }

  Stack buildContent(BuildContext context, BoxConstraints boxConstraints,
      Map<String, double> rowPositions) {
    return Stack(
      children: [
        RepaintBoundary(
          child: Column(
              children: notes
                  .map((note) => MIDINoteLane(
                        viewModel: widget.viewModel,
                        height: widget.viewModel.noteHeight,
                        note: note,
                        model: widget.model,
                        onPanStart: (details) =>
                            onPanStartBackground(context, details),
                        onPanUpdate: (details) =>
                            onPanUpdateBackground(context, details),
                        onPanCancel: onPanCancelBackground,
                        onPanEnd: (details) =>
                            onPanEndBackground(context, details, rowPositions),
                      ))
                  .toList()),
        ),
        ...widget.model.midiNotes
            .map((note) => MIDINoteView(
                  model: widget.model,
                  midiEditorViewModel: widget.viewModel,
                  note: note,
                  rowPositions: rowPositions,
                  height: widget.viewModel.noteHeight,
                  isSelected: widget.model.selectedNotes.contains(note),
                  parentWidth: boxConstraints.maxWidth - 110,
                  onTap: () => onTap(context, note),
                  onDragUpdate: (details) =>
                      onNoteDragUpdate(context, note, details),
                ))
            .toList(),
        SelectionOverlayView(model: widget.viewModel.selectionOverlayViewModel)
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

  onNoteDragUpdate(
      BuildContext context, MIDINoteModel note, DragUpdateDetails details) {
    var renderBox = context.findRenderObject() as RenderBox;
    var localPosition = renderBox.globalToLocal(details.globalPosition);

    var index = (localPosition.dy / (widget.viewModel.noteHeight + 1));
    var newNote = notes[index.toInt()];

    note.note = newNote;
  }

  onTap(BuildContext context, MIDINoteModel note) {
    widget.model.setSelectedNote(note);
    widget.viewModel.clearLastTapTime();
    FocusScope.of(context).requestFocus(focusNode);
  }

  onDelete() {
    for (var note in widget.model.selectedNotes.toList()) {
      widget.model.removeNote(note);
    }
  }

  void onPanStartBackground(BuildContext context, DragStartDetails details) {
    var renderBox = context.findRenderObject() as RenderBox;
    var localPosition = renderBox.globalToLocal(details.globalPosition);
    widget.viewModel.selectionOverlayViewModel.onPanStart(localPosition);
  }

  void onPanUpdateBackground(BuildContext context, DragUpdateDetails details) {
    var renderBox = context.findRenderObject() as RenderBox;
    var localPosition = renderBox.globalToLocal(details.globalPosition);
    widget.viewModel.selectionOverlayViewModel.onPanUpdate(localPosition);
  }

  void onPanEndBackground(BuildContext context, DragEndDetails details,
      Map<String, double> rowPositions) {
    widget.viewModel.onPanEnd(
        viewportWidth: context.size?.width ?? 0, rowPositions: rowPositions);
  }

  void onPanCancelBackground() {
    widget.viewModel.selectionOverlayViewModel.onPanCancel();
  }
}
