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

class MIDIClipModel = _MIDIClipModel with _$MIDIClipModel;

abstract class _MIDIClipModel with Store {
  @observable
  ObservableList<MIDINoteModel> midiNotes = ObservableList.of([]);

  @computed
  Map<String, List<MIDINoteModel>> get midiNoteMap {
    Map<String, List<MIDINoteModel>> result = {};

    for (var event in midiNotes) {
      var key = event.note.getSymbol();
      if (result[key] == null) {
        result[key] = [];
      }
      result[key]!.add(event);
    }

    return result;
  }

  @action
  void addEvent({required double time, required Note note}) {
    var event = MIDINoteModel();
    event.note = note;
    event.time = time;
    midiNotes.add(event);
  }
}

class MIDINoteModel = _MIDINoteModel with _$MIDINoteModel;

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
