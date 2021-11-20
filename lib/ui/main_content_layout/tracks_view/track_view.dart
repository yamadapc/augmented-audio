import 'package:flutter/material.dart';

import 'track_view/clip.dart';
import 'track_view/track_controls.dart';
import 'track_view/track_title.dart';

class JamTrackView extends StatelessWidget {
  final String title;
  final int index;

  const JamTrackView({
    Key? key,
    required this.title,
    required this.index,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return RepaintBoundary(
      child: ClipRect(
        child: Container(
          width: 120,
          decoration: const BoxDecoration(
              color: Color.fromRGBO(79, 79, 79, 1.0),
              border: Border(
                left: BorderSide(color: Color.fromRGBO(65, 65, 65, 0.0)),
                right: BorderSide(color: Color.fromRGBO(65, 65, 65, 1.0)),
              )),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              // Track heading
              TrackTitle(title: title, index: index),
              Expanded(
                child: SingleChildScrollView(
                  child: Column(children: const [
                    ClipView(title: "Clip 1"),
                    ClipView(title: "Clip 2"),
                    ClipView(title: "Clip 3"),
                    ClipView(title: "Clip 4"),
                    ClipSlot(),
                    ClipSlot(),
                    ClipSlot(),
                    ClipSlot(),
                    ClipSlot(),
                    ClipSlot(),
                  ]),
                ),
              ),
              const RepaintBoundary(child: TrackControls())
              // Clips
            ],
          ),
        ),
      ),
    );
  }
}
