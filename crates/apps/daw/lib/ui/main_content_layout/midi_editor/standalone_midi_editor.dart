import 'package:flutter/cupertino.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_editor.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_model.dart';

class StandaloneMIDIEditorView extends StatelessWidget {
  final MIDIClipModel model = MIDIClipModel();

  StandaloneMIDIEditorView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MIDIEditorView(model: model);
  }
}
