import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:graphx/graphx.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:mobx/mobx.dart';

class MetronomeSceneBack extends GSprite {
  MetronomeStateModel model;
  Color strokeColor;
  Dispose? subscription;

  MetronomeSceneBack(this.model, this.strokeColor);

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
    final playheadValue = model.playhead;
    final playheadMod1 = 1.0 - playheadValue % 1.0;

    final beatsPerBar = model.beatsPerBar;

    final height = stage?.stageHeight ?? 0.0;
    final width = stage?.stageWidth ?? 100.0;
    const padding = 5.0;
    final rectWidth =
        Math.min((width - padding * beatsPerBar) / beatsPerBar, height - 10.0);
    var left = (width - (rectWidth + padding) * beatsPerBar) / 2.0;
    final top = height / 2.0;

    final borderRadius = Platform.isAndroid ? rectWidth : rectWidth * 0.2;

    for (var i = 0; i < beatsPerBar; i++) {
      final isTick = playheadValue % beatsPerBar >= i &&
          playheadValue % beatsPerBar < (i + 1);
      final tickFactor = isTick ? .4 : .0;

      final offset = Offset(left + rectWidth / 2.0, top);

      if (isTick) {
        final Paint strokePaint = Paint();
        strokePaint.color = strokeColor.withOpacity(1.0 * playheadMod1);
        final rect =
            Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 3);
        final rrect =
            RRect.fromRectAndRadius(rect, Radius.circular(borderRadius));
        canvas.drawRRect(rrect, strokePaint);
      }

      final Paint paint = Paint();
      const baseColor = CupertinoColors.activeBlue;
      paint.color =
          baseColor.withOpacity(0.4 + 1.2 * playheadMod1 * tickFactor);
      final rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 5);
      final rrect =
          RRect.fromRectAndRadius(rect, Radius.circular(borderRadius));
      canvas.drawRRect(rrect, paint);
      left += rectWidth + padding;
    }
  }
}
