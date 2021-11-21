// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'audio_io_state.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$AudioIOState on _AudioIOState, Store {
  final _$availableInputsAtom = Atom(name: '_AudioIOState.availableInputs');

  @override
  ObservableList<AudioInput> get availableInputs {
    _$availableInputsAtom.reportRead();
    return super.availableInputs;
  }

  @override
  set availableInputs(ObservableList<AudioInput> value) {
    _$availableInputsAtom.reportWrite(value, super.availableInputs, () {
      super.availableInputs = value;
    });
  }

  @override
  String toString() {
    return '''
availableInputs: ${availableInputs}
    ''';
  }
}

mixin _$AudioInput on _AudioInput, Store {
  final _$idAtom = Atom(name: '_AudioInput.id');

  @override
  String get id {
    _$idAtom.reportRead();
    return super.id;
  }

  @override
  set id(String value) {
    _$idAtom.reportWrite(value, super.id, () {
      super.id = value;
    });
  }

  final _$titleAtom = Atom(name: '_AudioInput.title');

  @override
  String get title {
    _$titleAtom.reportRead();
    return super.title;
  }

  @override
  set title(String value) {
    _$titleAtom.reportWrite(value, super.title, () {
      super.title = value;
    });
  }

  @override
  String toString() {
    return '''
id: ${id},
title: ${title}
    ''';
  }
}
