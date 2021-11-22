import 'package:flutter_daw_mock_ui/state/entity.dart';
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

  static Note ofSymbol(String symbol) {
    var letter = symbol.substring(0, symbol.length - 1);
    var octave = int.parse(symbol[symbol.length - 1]);
    var noteIndex = noteSymbols.indexOf(letter);
    return Note(octave, noteIndex);
  }
}

class MIDIClipModel extends _MIDIClipModel with _$MIDIClipModel {
  @override
  ActionController get _$_MIDIClipModelActionController =>
      getActionController();
}

abstract class _MIDIClipModel with Store, Entity {
  @override
  String id = "MIDIClipModel:0";

  @observable
  ObservableList<MIDINoteModel> midiNotes = ObservableList.of([]);

  @observable
  ObservableList<MIDINoteModel> selectedNotes = ObservableList.of([]);

  @action
  void setSelectedNote(MIDINoteModel noteModel) {
    selectedNotes.clear();
    selectedNotes.add(noteModel);
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

class MIDINoteModel extends _MIDINoteModel with _$MIDINoteModel, Entity {
  @override
  String id;

  MIDINoteModel(this.id);
}

abstract class _MIDINoteModel with Store {
  @observable
  double time = 0;

  @observable
  double duration = (1 / 4) / 4;

  @observable
  double velocity = 0;

  @observable
  Note note = Note(3, 0);
}
