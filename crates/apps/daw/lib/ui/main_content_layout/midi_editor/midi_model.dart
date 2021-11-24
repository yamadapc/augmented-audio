import 'package:flutter_daw_mock_ui/state/entity.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:mobx/mobx.dart';

part 'midi_model.g.dart';

final List<String> noteSymbols = [
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
final List<int> sharpNotes = noteSymbols
    .asMap()
    .entries
    .where((e) => e.value.contains("#"))
    .map((e) => e.key)
    .toList();

@JsonSerializable()
class Note {
  /// If this is C3, octave is '3'
  final int octave;

  /// This is a number from 1-12.
  ///
  /// 1 is C, 2 is C#, etc.
  final int note;

  Note(this.octave, this.note);

  String getSymbol() {
    return "${noteSymbols[note]}$octave";
  }

  bool isSharp() {
    return sharpNotes.contains(note);
  }

  String toString() => getSymbol();

  Map<String, dynamic> toJson() => _$NoteToJson(this);
  factory Note.fromJson(Map<String, dynamic> json) => _$NoteFromJson(json);

  static Note ofSymbol(String symbol) {
    var letter = symbol.substring(0, symbol.length - 1);
    var octave = int.parse(symbol[symbol.length - 1]);
    var noteIndex = noteSymbols.indexOf(letter);
    return Note(octave, noteIndex);
  }
}

@JsonSerializable()
class MIDIClipModel extends _MIDIClipModel with _$MIDIClipModel {
  @override
  ActionController get _$_MIDIClipModelActionController =>
      getActionController();

  factory MIDIClipModel.fromJson(Map<String, dynamic> json) =>
      _$MIDIClipModelFromJson(json);

  Map<String, dynamic> toJson() => _$MIDIClipModelToJson(this);

  MIDIClipModel();
}

abstract class _MIDIClipModel with Store, Entity {
  @override
  String id = "MIDIClipModel:0";

  @JsonKey(
      toJson: noteObservableListToJSON, fromJson: noteObservableListFromJSON)
  @observable
  ObservableList<MIDINoteModel> midiNotes = ObservableList.of([]);

  @JsonKey(
      toJson: noteObservableListToJSON, fromJson: noteObservableListFromJSON)
  @observable
  ObservableList<MIDINoteModel> selectedNotes = ObservableList.of([]);

  @action
  void setSelectedNote(MIDINoteModel noteModel) {
    selectedNotes.clear();
    selectedNotes.add(noteModel);
  }

  @action
  void unselectNotes() {
    selectedNotes.clear();
  }

  @action
  void removeNote(MIDINoteModel noteModel) {
    selectedNotes.remove(noteModel);
    midiNotes.remove(noteModel);
  }

  @action
  void addEvent({required double time, required Note note}) {
    var event = MIDINoteModel(
        id + "/midiNotes/MidiNoteModel:" + midiNotes.length.toString());
    event.note = note;
    event.time = time;
    midiNotes.add(event);
  }
}

class MIDINoteModel extends _MIDINoteModel with _$MIDINoteModel {
  MIDINoteModel(String id) {
    this.id = id;
  }
}

@JsonSerializable(createFactory: false)
abstract class _MIDINoteModel with Store, Entity {
  @override
  String id = "";

  @observable
  double time = 0;

  @observable
  double duration = (1 / 4) / 4;

  @observable
  double velocity = 0;

  @observable
  Note note = Note(3, 0);
}

ObservableList<MIDINoteModel> noteObservableListFromJSON(List<dynamic> json) =>
    throw UnimplementedError("Not implemented");

List<dynamic> noteObservableListToJSON(ObservableList<MIDINoteModel> notes) {
  return notes.map((element) => _$MIDINoteModelToJson(element)).toList();
}
