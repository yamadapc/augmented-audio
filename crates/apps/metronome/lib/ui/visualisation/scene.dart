import 'dart:ui';

import 'package:flutter/cupertino.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

class MetronomeSceneBack extends GSprite {
  Observable<double> playhead;
  Dispose? subscription;

  MetronomeSceneBack(this.playhead);

  @override
  void addedToStage() {
    subscription = playhead.observe((_) {
      stage!.scene.requestRender();
    });
  }

  @override
  void removedFromStage() {
    subscription?.call();
  }

  @override
  void paint(Canvas canvas) {
    var playheadValue = playhead.value;
    var playheadPrime = 1.0 - playheadValue % 1.0;

    var width = stage?.stageWidth ?? 100.0;
    var padding = 5.0;
    var rectWidth = (width - padding * 2) / 4;
    var left = 0.0;
    var top = rectWidth / 2.0;

    for (var i = 0; i < 4; i++) {
      var isTick = playheadValue % 4.0 >= i && playheadValue % 4.0 < (i + 1);
      var tickFactor = (isTick ? .4 : .0);

      var offset = Offset(left + rectWidth / 2.0, top);

      if (isTick) {
        Paint strokePaint = Paint();
        strokePaint.color = Color.fromRGBO(255, 255, 255, 1.0 * playheadPrime);
        var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 3);
        var rrect = RRect.fromRectAndRadius(rect, const Radius.circular(10.0));
        canvas.drawRRect(rrect, strokePaint);
      }

      Paint paint = Paint();
      var baseColor = CupertinoColors.activeBlue;
      paint.color =
          baseColor.withOpacity(0.4 + 1.2 * playheadPrime * tickFactor);
      var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 5);
      var rrect = RRect.fromRectAndRadius(rect, const Radius.circular(10.0));
      canvas.drawRRect(rrect, paint);
      left += rectWidth + padding;
    }
  }
}
