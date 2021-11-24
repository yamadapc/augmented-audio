import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_model.dart';
import 'package:graphx/graphx.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:mobx/mobx.dart';

part 'midi_editor_view_model.g.dart';

class MIDIEditorViewModel extends _MIDIEditorViewModel
    with _$MIDIEditorViewModel {
  MIDIEditorViewModel({MIDIClipModel? midiClipModel}) {
    this.midiClipModel = midiClipModel ?? MIDIClipModel();
  }

  Map<String, dynamic> toJson() => _$MIDIEditorViewModelToJson(this);
}

@JsonSerializable(createFactory: false)
abstract class _MIDIEditorViewModel with Store {
  @observable
  MIDIClipModel midiClipModel = MIDIClipModel();

  @observable
  double representedBars = 4.0;

  @observable
  double noteHeight = 20.0;

  @observable
  int? lastTapTime = null;

  @action
  void setLastTapTime(int time) {
    lastTapTime = time;
  }

  @action
  void clearLastTapTime() {
    lastTapTime = null;
  }

  @action
  void resizeNotesByDelta(double delta) {
    var smallerDelta = delta * 0.1;
    noteHeight = Math.min(30, Math.max(noteHeight + smallerDelta, 5));
  }
}
