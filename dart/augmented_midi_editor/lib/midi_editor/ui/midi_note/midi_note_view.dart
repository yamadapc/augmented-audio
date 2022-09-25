import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../../midi_editor_view_model.dart';
import '../../midi_model.dart';
import 'midi_resize_handle_view.dart';

class MIDINoteView extends StatelessWidget {
  final MIDIClipModel model;
  final MIDIEditorViewModel midiEditorViewModel;
  final MIDINoteModel note;
  final double height;

  final Map<String, double> rowPositions;
  final double parentWidth;
  final void Function(DragUpdateDetails) onDragUpdate;
  final void Function() onTap;
  final bool isSelected;

  const MIDINoteView({
    Key? key,
    required this.model,
    required this.midiEditorViewModel,
    required this.note,
    required this.height,
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
                      onPanStart: onPanStart,
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
    // TODO - snap using the mouse position instead
    var dx = details.delta.dx / parentWidth;
    midiEditorViewModel.onNoteTimeDelta(note, dx);
    onDragUpdate(details);
  }

  void onPanStart(DragStartDetails details) {
    model.setSelectedNote(note);
  }
}
