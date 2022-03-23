import 'dart:async';

import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

part 'volume_meter.g.dart';

class VolumeMeterModel = _VolumeMeterModel with _$VolumeMeterModel;

abstract class _VolumeMeterModel with Store {
  @observable
  double volumeLeft = 0.0;

  @observable
  double volumeRight = 0.0;
}

class VolumeMeter extends StatefulWidget {
  const VolumeMeter({Key? key}) : super(key: key);

  @override
  State<VolumeMeter> createState() => _VolumeMeterState();
}

class _VolumeMeterState extends State<VolumeMeter> {
  final VolumeMeterModel model = VolumeMeterModel();

  double volume = 0.0;
  Matrix4 transform = Matrix4.identity();

  late Timer timer;

  @override
  void initState() {
    var tick = 0.0;
    timer = Timer.periodic(const Duration(milliseconds: 16), (d) {
      tick += 0.01;
      setState(() {
        volume = (Math.sin(tick) + 1) / 2;
        transform.setIdentity();
        transform.scale(1.0, volume);
      });
    });
    super.initState();
  }

  @override
  void deactivate() {
    timer.cancel();
    super.deactivate();
  }

  @override
  Widget build(BuildContext context) {
    return RepaintBoundary(
      child: RotatedBox(
        quarterTurns: 2,
        child: SizedBox(
            height: 150,
            child: Container(
              decoration: BoxDecoration(
                  border:
                      Border.all(color: const Color.fromRGBO(90, 90, 90, 1.0))),
              child: Transform(
                transform: transform,
                child: Container(
                    decoration: const BoxDecoration(
                      color: Colors.green,
                    ),
                    child: null),
              ),
            )),
      ),
    );
  }
}

class VolumeMeterScene extends GSprite {
  late GShape rectangleLeft;
  late GShape rectangleRight;
  var tick = 0.0;

  @override
  void addedToStage() {
    var backgroundLeft = GShape();
    backgroundLeft.graphics
      ..beginFill(const Color.fromRGBO(54, 54, 54, 1.0))
      ..drawRect(0, 0, 11.5, (stage?.stageHeight ?? 0))
      ..endFill();
    addChild(backgroundLeft);
    var backgroundRight = GShape();
    backgroundRight.graphics
      ..beginFill(const Color.fromRGBO(54, 54, 54, 1.0))
      ..drawRect(15.0, 0, 11.5, (stage?.stageHeight ?? 0))
      ..endFill();
    addChild(backgroundRight);

    var volumeWidth = 10.0;
    var volumeHeight = 40.0;
    rectangleLeft = GShape();
    rectangleLeft.graphics.lineStyle(1.0, Colors.green)
      ..beginFill(Colors.green)
      ..drawRect(2.5, (stage?.stageHeight ?? 0) - volumeHeight, volumeWidth,
          volumeHeight)
      ..endFill();
    addChild(rectangleLeft);

    rectangleRight = GShape();
    rectangleRight.graphics.lineStyle(1.0, Colors.green)
      ..beginFill(Colors.green)
      ..drawRect(volumeWidth + 5.0, (stage?.stageHeight ?? 0) - volumeHeight,
          volumeWidth, volumeHeight)
      ..endFill();
    addChild(rectangleRight);
  }

  @override
  void update(double delta) {
    super.update(delta);

    tick += delta;

    var volumeHeight = 40.0;
    var height = volumeHeight * (1 + Math.sin(tick * 4.0));
    rectangleLeft.height = height;
    rectangleLeft.y = stage?.stageHeight;
    rectangleLeft.pivotY = stage?.stageHeight ?? 0;

    height = volumeHeight * (1 + Math.cos(tick * 4.0));
    rectangleRight.height = height;
    rectangleRight.y = stage?.stageHeight;
    rectangleRight.pivotY = stage?.stageHeight ?? 0;
    // print(rectangle.y);
  }
}
