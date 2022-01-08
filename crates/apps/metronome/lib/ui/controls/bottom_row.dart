import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

import '../../bridge_generated.dart';

class BottomRow extends StatelessWidget {
  final Metronome metronome;
  final Observable<bool> isPlaying;

  BottomRow({Key? key, required this.metronome, required this.isPlaying})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Expanded(
            child: CupertinoButton(
                color: CupertinoColors.activeBlue,
                onPressed: () {
                  metronome.setIsPlaying(value: !isPlaying.value);
                  runInAction(() {
                    isPlaying.value = !isPlaying.value;
                  });
                },
                child: Observer(
                  builder: (_) => Text(isPlaying.value ? "Stop" : "Start",
                      style: const TextStyle(color: CupertinoColors.white)),
                )),
          )
        ]);
  }
}
