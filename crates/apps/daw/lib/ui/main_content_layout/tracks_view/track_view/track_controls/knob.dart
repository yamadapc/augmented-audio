import 'dart:math';

import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';

class Knob extends StatelessWidget {
  final double value;
  final Function(double) onChange;

  const Knob({Key? key, required this.value, required this.onChange})
      : super(key: key);

  final double size = 50.0;

  get center {
    return size / 2.0;
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: size,
      width: size,
      child: GestureDetector(
        onPanUpdate: onPanUpdate,
        child:
            CustomPaint(size: Size.square(size), painter: KnobPainter(value)),
      ),
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    Point center = Point(this.center, this.center);
    Point localPosition =
        Point(details.localPosition.dx, details.localPosition.dy);

    // TODO - fix this stuff to not have conditional branches
    var startAngle = 0.75 * Math.PI;
    var sweep_angle = 0.75 * Math.PI_2;
    var scope = (sweep_angle + startAngle) - startAngle;

    var angle =
        atan2(center.y - localPosition.y, center.x - localPosition.x) + Math.PI;
    if (angle > 0.25 * Math.PI && angle < 0.75 * Math.PI) {
      return;
    } else if (angle <= 0.25 * Math.PI) {
      angle = Math.PI_2 - startAngle + angle;
    } else {
      angle -= startAngle;
    }

    var newValue = angle / scope;
    newValue = Math.max(0, Math.min(newValue, 1));
    onChange(newValue);
  }
}

class KnobPainter extends CustomPainter {
  final double value;

  KnobPainter(this.value);

  @override
  void paint(Canvas canvas, Size size) {
    var strokeWidth = size.width / 15;
    var center = Offset.zero.translate(size.width / 2, size.width / 2);
    var radius = size.width / 2 - strokeWidth;

    var rect = Rect.fromCircle(center: center, radius: radius);

    void drawArc(Color color, double filled) {
      var paint = Paint();
      paint.color = color;
      paint.style = PaintingStyle.stroke;
      paint.strokeWidth = strokeWidth;

      var coverage = 0.75;
      var startAngle = coverage * Math.PI;
      var sweep_angle = filled * coverage * Math.PI_2;
      canvas.drawArc(rect, startAngle, sweep_angle, false, paint);
    }

    drawArc(Colors.black, 1.0);
    drawArc(Colors.blue, value);
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
