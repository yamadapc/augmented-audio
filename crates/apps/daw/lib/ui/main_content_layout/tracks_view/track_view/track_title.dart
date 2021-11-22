import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

class TrackTitle extends StatelessWidget {
  const TrackTitle({
    Key? key,
    required this.track,
    required this.index,
  }) : super(key: key);

  final Track track;
  final int index;

  @override
  Widget build(BuildContext context) {
    return ReorderableDragStartListener(
        index: index,
        child: GestureDetector(
          onTap: () {
            track.select();
          },
          child: SizedBox(
            width: double.infinity,
            child: Observer(
              builder: (_) => Container(
                padding: const EdgeInsets.all(8.0),
                decoration: BoxDecoration(
                    color: Colors.white38,
                    border: track.isSelected
                        ? Border.all(color: Colors.blue)
                        : const Border(
                            top: BorderSide(color: Colors.transparent),
                            left: BorderSide(color: Colors.transparent),
                            right: BorderSide(color: Colors.transparent),
                            bottom: BorderSide(color: Colors.black))),
                child: Text(
                  track.title,
                ),
              ),
            ),
          ),
        ));
  }
}
