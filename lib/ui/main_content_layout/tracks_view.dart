import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:graphx/graphx.dart';

import 'tracks_view/track_view.dart';

class TracksView extends StatefulWidget {
  const TracksView({Key? key}) : super(key: key);

  @override
  State<TracksView> createState() => _TracksViewState();
}

class _TracksViewState extends State<TracksView> {
  var tracks = [
    Track("1", "Track 1"),
    Track("2", "Track 2"),
    Track("3", "Track 3"),
    Track("4", "Track 4"),
  ];

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (BuildContext context, BoxConstraints viewportConstraints) {
        var trackViews = List.generate(tracks.length, (trackIndex) {
          var track = tracks[trackIndex];
          return JamTrackView(
              key: Key(track.id), title: track.title, index: trackIndex);
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
    setState(() {
      var elem = tracks[sourceIndex];
      tracks.removeAt(sourceIndex);
      var targetPrime = Math.max(
          sourceIndex < targetIndex ? targetIndex - 1 : targetIndex, 0);
      tracks.insert(targetPrime, elem);
    });
  }
}
