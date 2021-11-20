const noteSymbols = [
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

class Note {
  /// If this is C3, octave is '3'
  final int octave;

  /// This is a number from 1-12.
  ///
  /// 1 is C, 2 is C#, etc.
  final int note;

  Note(this.octave, this.note);
}
