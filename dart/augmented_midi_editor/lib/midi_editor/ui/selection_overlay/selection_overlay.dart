import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'model.dart';

class SelectionOverlayView extends StatelessWidget {
  final SelectionOverlayViewModel model;

  const SelectionOverlayView({Key? key, required this.model}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        if (!model.isDragging) {
          return Container(child: null);
        }

        var boundingBox = model.boundingBox!;
        return Positioned(
            top: boundingBox.top,
            left: boundingBox.left,
            child: Container(
                width: boundingBox.width,
                height: boundingBox.height,
                decoration: BoxDecoration(
                    border: Border.all(color: Colors.blue.withOpacity(0.8)),
                    color: Colors.blue.withOpacity(0.1)),
                child: null));
      },
    );
  }
}
