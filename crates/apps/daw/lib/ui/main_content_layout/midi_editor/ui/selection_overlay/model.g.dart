// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

Map<String, dynamic> _$SelectionOverlayViewModelToJson(
        _SelectionOverlayViewModel instance) =>
    <String, dynamic>{
      'startingPoint': pointToJson(instance.startingPoint),
      'currentPoint': pointToJson(instance.currentPoint),
    };

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$SelectionOverlayViewModel on _SelectionOverlayViewModel, Store {
  Computed<bool>? _$isDraggingComputed;

  @override
  bool get isDragging =>
      (_$isDraggingComputed ??= Computed<bool>(() => super.isDragging,
              name: '_SelectionOverlayViewModel.isDragging'))
          .value;
  Computed<Rectangle<double>?>? _$boundingBoxComputed;

  @override
  Rectangle<double>? get boundingBox => (_$boundingBoxComputed ??=
          Computed<Rectangle<double>?>(() => super.boundingBox,
              name: '_SelectionOverlayViewModel.boundingBox'))
      .value;

  final _$startingPointAtom =
      Atom(name: '_SelectionOverlayViewModel.startingPoint');

  @override
  Point<double>? get startingPoint {
    _$startingPointAtom.reportRead();
    return super.startingPoint;
  }

  @override
  set startingPoint(Point<double>? value) {
    _$startingPointAtom.reportWrite(value, super.startingPoint, () {
      super.startingPoint = value;
    });
  }

  final _$currentPointAtom =
      Atom(name: '_SelectionOverlayViewModel.currentPoint');

  @override
  Point<double>? get currentPoint {
    _$currentPointAtom.reportRead();
    return super.currentPoint;
  }

  @override
  set currentPoint(Point<double>? value) {
    _$currentPointAtom.reportWrite(value, super.currentPoint, () {
      super.currentPoint = value;
    });
  }

  final _$_SelectionOverlayViewModelActionController =
      ActionController(name: '_SelectionOverlayViewModel');

  @override
  void onPanStart(Offset offset) {
    final _$actionInfo = _$_SelectionOverlayViewModelActionController
        .startAction(name: '_SelectionOverlayViewModel.onPanStart');
    try {
      return super.onPanStart(offset);
    } finally {
      _$_SelectionOverlayViewModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void onPanUpdate(Offset offset) {
    final _$actionInfo = _$_SelectionOverlayViewModelActionController
        .startAction(name: '_SelectionOverlayViewModel.onPanUpdate');
    try {
      return super.onPanUpdate(offset);
    } finally {
      _$_SelectionOverlayViewModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void onPanCancel() {
    final _$actionInfo = _$_SelectionOverlayViewModelActionController
        .startAction(name: '_SelectionOverlayViewModel.onPanCancel');
    try {
      return super.onPanCancel();
    } finally {
      _$_SelectionOverlayViewModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
startingPoint: ${startingPoint},
currentPoint: ${currentPoint},
isDragging: ${isDragging},
boundingBox: ${boundingBox}
    ''';
  }
}
