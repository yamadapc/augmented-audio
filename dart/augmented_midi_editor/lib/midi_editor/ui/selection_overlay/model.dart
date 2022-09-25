import 'dart:math';

import 'package:graphx/graphx.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:mobx/mobx.dart';

part 'model.g.dart';

class SelectionOverlayViewModel extends _SelectionOverlayViewModel
    with _$SelectionOverlayViewModel {
  Map<String, dynamic> toJson() => _$SelectionOverlayViewModelToJson(this);
}

@JsonSerializable(createFactory: false)
abstract class _SelectionOverlayViewModel with Store {
  @JsonKey(toJson: pointToJson, fromJson: pointFromJson)
  @observable
  Point<double>? startingPoint;

  @JsonKey(toJson: pointToJson, fromJson: pointFromJson)
  @observable
  Point<double>? currentPoint;

  @JsonKey(ignore: true)
  @computed
  bool get isDragging => startingPoint != null && currentPoint != null;

  @JsonKey(ignore: true)
  @computed
  Rectangle<double>? get boundingBox {
    if (!isDragging) {
      return null;
    }

    double left = Math.min(startingPoint!.x, currentPoint!.x);
    double top = Math.min(startingPoint!.y, currentPoint!.y);
    double width = (startingPoint!.x - currentPoint!.x).abs();
    double height = (startingPoint!.y - currentPoint!.y).abs();

    Rectangle<double> result = Rectangle<double>(left, top, width, height);
    return result;
  }

  @action
  void onPanStart(Offset offset) {
    startingPoint = Point(offset.dx, offset.dy);
  }

  @action
  void onPanUpdate(Offset offset) {
    currentPoint = Point(offset.dx, offset.dy);
  }

  @action
  void onPanCancel() {
    startingPoint = null;
    currentPoint = null;
  }
}

Map<String, dynamic>? pointToJson(Point<double>? point) {
  if (point == null) {
    return null;
  }

  Map<String, double> result = {};
  result["x"] = point.x;
  result["y"] = point.y;
  return result;
}

Point<double>? pointFromJson(Map<String, dynamic>? json) {
  if (json == null) {
    return null;
  }

  var result = Point<double>(json["x"], json["y"]);
  return result;
}
