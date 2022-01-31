import 'dart:ui';

import 'package:flutter/cupertino.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

import '../../modules/state/metronome_state_model.dart';

class MetronomeSceneBack extends GSprite {
  MetronomeStateModel model;
  Dispose? subscription;

  MetronomeSceneBack(this.model);

  @override
  void addedToStage() {
    subscription = reaction((_) => model.playhead + model.beatsPerBar, (_) {
      stage!.scene.requestRender();
    });
  }

  @override
  void removedFromStage() {
    subscription?.call();
  }

  @override
  void paint(Canvas canvas) {
    var playheadValue = model.playhead;
    var playheadMod1 = 1.0 - playheadValue % 1.0;

    var beatsPerBar = model.beatsPerBar;

    var height = stage?.stageHeight ?? 0.0;
    var width = stage?.stageWidth ?? 100.0;
    var padding = 5.0;
    var rectWidth =
        Math.min((width - padding * beatsPerBar) / beatsPerBar, height - 10.0);
    var left = (width - (rectWidth + padding) * beatsPerBar) / 2.0;
    var top = height / 2.0;

    var borderRadius = rectWidth * 0.2;

    for (var i = 0; i < beatsPerBar; i++) {
      var isTick = playheadValue % beatsPerBar >= i &&
          playheadValue % beatsPerBar < (i + 1);
      var tickFactor = (isTick ? .4 : .0);

      var offset = Offset(left + rectWidth / 2.0, top);

      if (isTick) {
        Paint strokePaint = Paint();
        strokePaint.color =
            CupertinoColors.white.withOpacity(1.0 * playheadMod1);
        var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 3);
        var rrect =
            RRect.fromRectAndRadius(rect, Radius.circular(borderRadius));
        canvas.drawRRect(rrect, strokePaint);
      }

      Paint paint = Paint();
      var baseColor = CupertinoColors.activeBlue;
      paint.color =
          baseColor.withOpacity(0.4 + 1.2 * playheadMod1 * tickFactor);
      var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 5);
      var rrect = RRect.fromRectAndRadius(rect, Radius.circular(borderRadius));
      canvas.drawRRect(rrect, paint);
      left += rectWidth + padding;
    }
  }
}
