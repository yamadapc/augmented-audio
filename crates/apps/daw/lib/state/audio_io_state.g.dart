// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'audio_io_state.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$AudioIOState on _AudioIOState, Store {
  final _$currentInputDeviceAtom =
      Atom(name: '_AudioIOState.currentInputDevice');

  @override
  AudioDevice? get currentInputDevice {
    _$currentInputDeviceAtom.reportRead();
    return super.currentInputDevice;
  }

  @override
  set currentInputDevice(AudioDevice? value) {
    _$currentInputDeviceAtom.reportWrite(value, super.currentInputDevice, () {
      super.currentInputDevice = value;
    });
  }

  final _$currentOutputDeviceAtom =
      Atom(name: '_AudioIOState.currentOutputDevice');

  @override
  AudioDevice? get currentOutputDevice {
    _$currentOutputDeviceAtom.reportRead();
    return super.currentOutputDevice;
  }

  @override
  set currentOutputDevice(AudioDevice? value) {
    _$currentOutputDeviceAtom.reportWrite(value, super.currentOutputDevice, () {
      super.currentOutputDevice = value;
    });
  }

  final _$inputDevicesAtom = Atom(name: '_AudioIOState.inputDevices');

  @override
  ObservableList<AudioDevice> get inputDevices {
    _$inputDevicesAtom.reportRead();
    return super.inputDevices;
  }

  @override
  set inputDevices(ObservableList<AudioDevice> value) {
    _$inputDevicesAtom.reportWrite(value, super.inputDevices, () {
      super.inputDevices = value;
    });
  }

  final _$outputDevicesAtom = Atom(name: '_AudioIOState.outputDevices');

  @override
  ObservableList<AudioDevice> get outputDevices {
    _$outputDevicesAtom.reportRead();
    return super.outputDevices;
  }

  @override
  set outputDevices(ObservableList<AudioDevice> value) {
    _$outputDevicesAtom.reportWrite(value, super.outputDevices, () {
      super.outputDevices = value;
    });
  }

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

  final _$_AudioIOStateActionController =
      ActionController(name: '_AudioIOState');

  @override
  void setInputDevice(AudioDevice? inputDevice) {
    final _$actionInfo = _$_AudioIOStateActionController.startAction(
        name: '_AudioIOState.setInputDevice');
    try {
      return super.setInputDevice(inputDevice);
    } finally {
      _$_AudioIOStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setOutputDevice(AudioDevice? outputDevice) {
    final _$actionInfo = _$_AudioIOStateActionController.startAction(
        name: '_AudioIOState.setOutputDevice');
    try {
      return super.setOutputDevice(outputDevice);
    } finally {
      _$_AudioIOStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
currentInputDevice: ${currentInputDevice},
currentOutputDevice: ${currentOutputDevice},
inputDevices: ${inputDevices},
outputDevices: ${outputDevices},
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
