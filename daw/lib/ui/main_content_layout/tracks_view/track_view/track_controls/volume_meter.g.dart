// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'volume_meter.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$VolumeMeterModel on _VolumeMeterModel, Store {
  final _$volumeLeftAtom = Atom(name: '_VolumeMeterModel.volumeLeft');

  @override
  double get volumeLeft {
    _$volumeLeftAtom.reportRead();
    return super.volumeLeft;
  }

  @override
  set volumeLeft(double value) {
    _$volumeLeftAtom.reportWrite(value, super.volumeLeft, () {
      super.volumeLeft = value;
    });
  }

  final _$volumeRightAtom = Atom(name: '_VolumeMeterModel.volumeRight');

  @override
  double get volumeRight {
    _$volumeRightAtom.reportRead();
    return super.volumeRight;
  }

  @override
  set volumeRight(double value) {
    _$volumeRightAtom.reportWrite(value, super.volumeRight, () {
      super.volumeRight = value;
    });
  }

  @override
  String toString() {
    return '''
volumeLeft: ${volumeLeft},
volumeRight: ${volumeRight}
    ''';
  }
}
