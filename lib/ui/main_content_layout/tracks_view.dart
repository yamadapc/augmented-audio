import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:graphx/graphx.dart';

import 'tracks_view/track_view.dart';

class TracksView extends StatelessWidget {
  final TracksList tracksList;

  TracksView({Key? key, required this.tracksList}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        var trackViews = List.generate(tracksList.tracks.length, (trackIndex) {
          var track = tracksList.tracks[trackIndex];
          return JamTrackView(
              key: Key(track.id), track: track, index: trackIndex);
        }).toList();

        var content = ReorderableListView(
          onReorder: onReorderTracks,
          physics: const BouncingScrollPhysics(
              parent: AlwaysScrollableScrollPhysics()),
          scrollDirection: Axis.horizontal,
          children: trackViews,
          buildDefaultDragHandles: false,
        );

        return content;
      },
    );
  }

  void onReorderTracks(int sourceIndex, int targetIndex) {
    tracksList.reorderTracks(sourceIndex, targetIndex);
  }
}
