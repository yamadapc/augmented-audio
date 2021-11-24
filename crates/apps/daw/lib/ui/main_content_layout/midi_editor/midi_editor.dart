import 'package:flutter/material.dart';

import 'midi_model.dart';
import 'ui/background/midi_timeline_background.dart';
import 'ui/editor_content.dart';
import 'ui/header/midi_timeline_header.dart';

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
                    child: MIDIEditorContentView(model: model),
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
