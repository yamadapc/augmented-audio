// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'midi_editor_view_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Map<String, dynamic> _$MIDIEditorViewModelToJson(
        _MIDIEditorViewModel instance) =>
    <String, dynamic>{
      'midiClipModel': instance.midiClipModel,
      'representedBars': instance.representedBars,
      'noteHeight': instance.noteHeight,
    };

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$MIDIEditorViewModel on _MIDIEditorViewModel, Store {
  final _$midiClipModelAtom = Atom(name: '_MIDIEditorViewModel.midiClipModel');

  @override
  MIDIClipModel get midiClipModel {
    _$midiClipModelAtom.reportRead();
    return super.midiClipModel;
  }

  @override
  set midiClipModel(MIDIClipModel value) {
    _$midiClipModelAtom.reportWrite(value, super.midiClipModel, () {
      super.midiClipModel = value;
    });
  }

  final _$representedBarsAtom =
      Atom(name: '_MIDIEditorViewModel.representedBars');

  @override
  double get representedBars {
    _$representedBarsAtom.reportRead();
    return super.representedBars;
  }

  @override
  set representedBars(double value) {
    _$representedBarsAtom.reportWrite(value, super.representedBars, () {
      super.representedBars = value;
    });
  }

  final _$noteHeightAtom = Atom(name: '_MIDIEditorViewModel.noteHeight');

  @override
  double get noteHeight {
    _$noteHeightAtom.reportRead();
    return super.noteHeight;
  }

  @override
  set noteHeight(double value) {
    _$noteHeightAtom.reportWrite(value, super.noteHeight, () {
      super.noteHeight = value;
    });
  }

  final _$_MIDIEditorViewModelActionController =
      ActionController(name: '_MIDIEditorViewModel');

  @override
  void resizeNotesByDelta(double delta) {
    final _$actionInfo = _$_MIDIEditorViewModelActionController.startAction(
        name: '_MIDIEditorViewModel.resizeNotesByDelta');
    try {
      return super.resizeNotesByDelta(delta);
    } finally {
      _$_MIDIEditorViewModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
midiClipModel: ${midiClipModel},
representedBars: ${representedBars},
noteHeight: ${noteHeight}
    ''';
  }
}
