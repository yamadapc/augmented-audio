import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_daw_mock_ui/ui/common/scroll/interactive_viewer.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:graphx/graphx.dart';
import 'package:vector_math/vector_math_64.dart';

import 'tracks_view/track_view.dart';

class TracksView extends StatelessWidget {
  final TracksList tracksList;

  TracksView({Key? key, required this.tracksList}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var content = CustomInteractiveViewer.builder(
      builder: (context, viewport) => IntrinsicHeight(
        child: Observer(
          builder: (_) => Row(
              children: List.generate(tracksList.tracks.length, (trackIndex) {
            var track = tracksList.tracks[trackIndex];
            return JamTrackView(
                key: Key(track.id), track: track, index: trackIndex);
          }).toList()),
        ),
      ),
      useMouseWheelPan: true,
      scaleEnabled: false,
    );

    return content;
  }

  void onReorderTracks(int sourceIndex, int targetIndex) {
    tracksList.reorderTracks(sourceIndex, targetIndex);
  }

  double getHeight(Quad viewport) {
    var y1 = viewport.point0;
    var y2 = viewport.point3;
    var distance = y2.distanceTo(y1);
    return distance;
  }
}
