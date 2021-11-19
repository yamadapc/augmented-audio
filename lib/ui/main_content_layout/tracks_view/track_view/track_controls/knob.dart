import 'dart:math';

import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';

class Knob extends StatefulWidget {
  const Knob({Key? key}) : super(key: key);

  @override
  State<Knob> createState() => _KnobState();
}

class _KnobState extends State<Knob> {
  final double size = 50.0;
  get center {
    return size / 2.0;
  }

  double value = 0.0;
  bool isDragging = false;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: size,
      width: size,
      child: GestureDetector(
        onPanStart: onPanStart,
        onPanUpdate: onPanUpdate,
        onPanEnd: onPanEnd,
        child:
            CustomPaint(size: Size.square(size), painter: KnobPainter(value)),
      ),
    );
  }

  void onPanStart(DragStartDetails details) {
    setState(() {
      isDragging = true;
    });
  }

  void onPanUpdate(DragUpdateDetails details) {
    Point center = Point(this.center, this.center);
    Point localPosition =
        Point(details.localPosition.dx, details.localPosition.dy);

    // TODO - fix this stuff to not have conditional branches
    var startAngle = 0.75 * Math.PI;
    var sweepAngle = 0.75 * Math.PI_2;
    var scope = (sweepAngle + startAngle) - startAngle;

    var angle =
        atan2(center.y - localPosition.y, center.x - localPosition.x) + Math.PI;
    if (angle > 0.25 * Math.PI && angle < 0.75 * Math.PI) {
      return;
    } else if (angle <= 0.25 * Math.PI) {
      angle = Math.PI_2 - startAngle + angle;
    } else {
      angle -= startAngle;
    }

    setState(() {
      value = angle / scope;
      value = Math.max(0, Math.min(value, 1));
    });
  }

  void onPanEnd(DragEndDetails details) {
    setState(() {
      isDragging = false;
    });
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
      var sweepAngle = filled * coverage * Math.PI_2;
      canvas.drawArc(rect, startAngle, sweepAngle, false, paint);
    }

    drawArc(Colors.black, 1.0);
    drawArc(Colors.blue, value);
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}
