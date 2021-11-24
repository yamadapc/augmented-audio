// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'midi_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Note _$NoteFromJson(Map<String, dynamic> json) => Note(
      json['octave'] as int,
      json['note'] as int,
    );

Map<String, dynamic> _$NoteToJson(Note instance) => <String, dynamic>{
      'octave': instance.octave,
      'note': instance.note,
    };

MIDIClipModel _$MIDIClipModelFromJson(Map<String, dynamic> json) =>
    MIDIClipModel()
      ..id = json['id'] as String
      ..midiNotes = noteObservableListFromJSON(json['midiNotes'] as List)
      ..selectedNotes =
          noteObservableListFromJSON(json['selectedNotes'] as List);

Map<String, dynamic> _$MIDIClipModelToJson(MIDIClipModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'midiNotes': noteObservableListToJSON(instance.midiNotes),
      'selectedNotes': noteObservableListToJSON(instance.selectedNotes),
    };

Map<String, dynamic> _$MIDINoteModelToJson(_MIDINoteModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'time': instance.time,
      'duration': instance.duration,
      'velocity': instance.velocity,
      'note': instance.note,
    };

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$MIDIClipModel on _MIDIClipModel, Store {
  final _$midiNotesAtom = Atom(name: '_MIDIClipModel.midiNotes');

  @override
  ObservableList<MIDINoteModel> get midiNotes {
    _$midiNotesAtom.reportRead();
    return super.midiNotes;
  }

  @override
  set midiNotes(ObservableList<MIDINoteModel> value) {
    _$midiNotesAtom.reportWrite(value, super.midiNotes, () {
      super.midiNotes = value;
    });
  }

  final _$selectedNotesAtom = Atom(name: '_MIDIClipModel.selectedNotes');

  @override
  ObservableList<MIDINoteModel> get selectedNotes {
    _$selectedNotesAtom.reportRead();
    return super.selectedNotes;
  }

  @override
  set selectedNotes(ObservableList<MIDINoteModel> value) {
    _$selectedNotesAtom.reportWrite(value, super.selectedNotes, () {
      super.selectedNotes = value;
    });
  }

  final _$_MIDIClipModelActionController =
      ActionController(name: '_MIDIClipModel');

  @override
  void setSelectedNote(MIDINoteModel noteModel) {
    final _$actionInfo = _$_MIDIClipModelActionController.startAction(
        name: '_MIDIClipModel.setSelectedNote');
    try {
      return super.setSelectedNote(noteModel);
    } finally {
      _$_MIDIClipModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void unselectNotes() {
    final _$actionInfo = _$_MIDIClipModelActionController.startAction(
        name: '_MIDIClipModel.unselectNotes');
    try {
      return super.unselectNotes();
    } finally {
      _$_MIDIClipModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void removeNote(MIDINoteModel noteModel) {
    final _$actionInfo = _$_MIDIClipModelActionController.startAction(
        name: '_MIDIClipModel.removeNote');
    try {
      return super.removeNote(noteModel);
    } finally {
      _$_MIDIClipModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void addEvent({required double time, required Note note}) {
    final _$actionInfo = _$_MIDIClipModelActionController.startAction(
        name: '_MIDIClipModel.addEvent');
    try {
      return super.addEvent(time: time, note: note);
    } finally {
      _$_MIDIClipModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
midiNotes: ${midiNotes},
selectedNotes: ${selectedNotes}
    ''';
  }
}

mixin _$MIDINoteModel on _MIDINoteModel, Store {
  final _$timeAtom = Atom(name: '_MIDINoteModel.time');

  @override
  double get time {
    _$timeAtom.reportRead();
    return super.time;
  }

  @override
  set time(double value) {
    _$timeAtom.reportWrite(value, super.time, () {
      super.time = value;
    });
  }

  final _$durationAtom = Atom(name: '_MIDINoteModel.duration');

  @override
  double get duration {
    _$durationAtom.reportRead();
    return super.duration;
  }

  @override
  set duration(double value) {
    _$durationAtom.reportWrite(value, super.duration, () {
      super.duration = value;
    });
  }

  final _$velocityAtom = Atom(name: '_MIDINoteModel.velocity');

  @override
  double get velocity {
    _$velocityAtom.reportRead();
    return super.velocity;
  }

  @override
  set velocity(double value) {
    _$velocityAtom.reportWrite(value, super.velocity, () {
      super.velocity = value;
    });
  }

  final _$noteAtom = Atom(name: '_MIDINoteModel.note');

  @override
  Note get note {
    _$noteAtom.reportRead();
    return super.note;
  }

  @override
  set note(Note value) {
    _$noteAtom.reportWrite(value, super.note, () {
      super.note = value;
    });
  }

  @override
  String toString() {
    return '''
time: ${time},
duration: ${duration},
velocity: ${velocity},
note: ${note}
    ''';
  }
}
