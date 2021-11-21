import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';

import 'track_view/clip.dart';
import 'track_view/track_controls.dart';
import 'track_view/track_title.dart';

class JamTrackView extends StatelessWidget {
  final int index;
  final Track track;

  const JamTrackView({
    Key? key,
    required this.track,
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
              TrackTitle(track: track, index: index),
              Expanded(
                child: Column(children: [
                  ...track.clips
                      .map((clip) => ClipView(title: clip.title))
                      .toList(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                  const ClipSlot(),
                ]),
              ),
              RepaintBoundary(child: TrackControls(track: track))
              // Clips
            ],
          ),
        ),
      ),
    );
  }
}
