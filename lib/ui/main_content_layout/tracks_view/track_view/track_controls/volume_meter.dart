import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';

class VolumeMeter extends StatelessWidget {
  const VolumeMeter({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var sceneBuilderWidget = SceneBuilderWidget(
        builder: () => SceneController(
              config: SceneConfig.autoRender,
              back: VolumeMeterScene(),
            ));
    return RepaintBoundary(
      child: SizedBox(
          height: 150,
          child: Container(
              decoration: BoxDecoration(
                  border:
                      Border.all(color: const Color.fromRGBO(90, 90, 90, 1.0))),
              child: null // sceneBuilderWidget,
              )),
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
