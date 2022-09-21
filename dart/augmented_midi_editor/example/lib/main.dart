import 'package:augmented_midi_editor/midi_editor/midi_editor_view_model.dart';
import 'package:flutter/cupertino.dart';
import 'package:augmented_midi_editor/midi_editor/midi_editor.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return CupertinoApp(
      title: 'Flutter Demo',
      home: MIDIEditorView(model: MIDIEditorViewModel()),
    );
  }
}
