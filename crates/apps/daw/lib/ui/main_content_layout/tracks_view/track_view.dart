import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';

import 'track_view/clip.dart';

class JamTrackView extends StatelessWidget {
  final Track track;

  const JamTrackView({
    Key? key,
    required this.track,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var trackWidth = 120.0;

    return RepaintBoundary(
      child: ClipRect(
        child: Container(
          width: trackWidth,
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
              const SizedBox(height: 30),
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
              const SizedBox(height: 300),
              // Clips
            ],
          ),
        ),
      ),
    );
  }
}
