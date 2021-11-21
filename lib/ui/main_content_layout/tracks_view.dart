import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_daw_mock_ui/ui/common/scroll/interactive_viewer.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';
import 'package:vector_math/vector_math_64.dart';

import 'tracks_view/track_view.dart';
import 'tracks_view/track_view/track_controls.dart';
import 'tracks_view/track_view/track_title.dart';

class TracksView extends StatelessWidget {
  final TracksList tracksList;
  final TransformationController transformationController =
      TransformationController();
  final Observable<Matrix4> translationXTransform =
      Observable(Matrix4.identity());

  TracksView({Key? key, required this.tracksList}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var content = CustomInteractiveViewer.builder(
      transformationController: transformationController,
      onScroll: (_) => onScroll(),
      builder: (context, viewport) => IntrinsicHeight(
        child: ConstrainedBox(
          constraints: BoxConstraints(minHeight: getHeight(viewport)),
          child: Observer(
            builder: (_) => Row(
                children: List.generate(tracksList.tracks.length, (trackIndex) {
              var track = tracksList.tracks[trackIndex];
              return JamTrackView(
                  key: Key(track.id), track: track, index: trackIndex);
            }).toList()),
          ),
        ),
      ),
      useMouseWheelPan: true,
      scaleEnabled: false,
    );

    return Stack(children: [
      content,
      Positioned(
          left: 0,
          top: 0,
          child: Observer(
              builder: (_) => Transform(
                  transform: translationXTransform.value,
                  child: Row(
                    children: tracksList.tracks
                        .asMap()
                        .entries
                        .map(
                          (entry) => Container(
                              decoration: const BoxDecoration(
                                  color: Color.fromRGBO(79, 79, 79, 1.0),
                                  border: Border(
                                    left: BorderSide(
                                        color: Color.fromRGBO(65, 65, 65, 0.0)),
                                    right: BorderSide(
                                        color: Color.fromRGBO(65, 65, 65, 1.0)),
                                  )),
                              width: 120,
                              child: IntrinsicHeight(
                                  child: TrackTitle(
                                      track: entry.value, index: entry.key))),
                        )
                        .toList(),
                  )))),
      Positioned(
        left: 0,
        bottom: 0,
        child: Observer(
          builder: (_) => Transform(
              transform: translationXTransform.value,
              child: Row(
                children: tracksList.tracks
                    .map(
                      (track) => Container(
                        decoration: const BoxDecoration(
                            color: Color.fromRGBO(79, 79, 79, 1.0),
                            border: Border(
                              left: BorderSide(
                                  color: Color.fromRGBO(65, 65, 65, 0.0)),
                              right: BorderSide(
                                  color: Color.fromRGBO(65, 65, 65, 1.0)),
                            )),
                        width: 120,
                        child: IntrinsicHeight(
                            child: RepaintBoundary(
                                child: TrackControls(track: track))),
                      ),
                    )
                    .toList(),
              )),
        ),
      )
    ]);
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

  void onScroll() {
    var value = transformationController.value.getTranslation().x;
    var newTransform = Matrix4.identity();
    newTransform.translate(value);
    runInAction(() {
      translationXTransform.value = newTransform;
    });
  }
}
