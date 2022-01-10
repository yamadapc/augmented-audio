import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:metronome/modules/history/session_entity.dart';

String leftPad(String target, int length) {
  final diff = length - target.length;
  for (var i = 0; i < diff; i++) {
    target = "0" + target;
  }
  return target;
}

String formatDuration(Duration duration) {
  var hours = duration.inHours;
  var minutes = duration.inMinutes;
  var seconds = duration.inSeconds;
  if (hours == 0) {
    return "${leftPad(minutes.toString(), 2)}:${leftPad(seconds.toString(), 2)}";
  }
  return "${leftPad(hours.toString(), 2)}:${leftPad(minutes.toString(), 2)}:${leftPad(seconds.toString(), 2)}";
}

class HistoryListItem extends StatelessWidget {
  final Session session;

  const HistoryListItem({Key? key, required this.session}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    DateFormat dateFormat = DateFormat("yyyy-MM-dd HH:mm");
    final formattedDate = dateFormat
        .format(DateTime.fromMillisecondsSinceEpoch(session.timestampMs));
    final duration = Duration(milliseconds: session.durationMs);

    return Container(
        padding: EdgeInsets.all(8.0),
        decoration: const BoxDecoration(
          border: Border(
              bottom: BorderSide(color: CupertinoColors.opaqueSeparator)),
        ),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text(formattedDate,
              style: const TextStyle(
                  fontSize: 14, color: CupertinoColors.secondaryLabel)),
          Row(children: [
            Expanded(
              child: Text(formatDuration(duration),
                  style: const TextStyle(
                      fontSize: 24, fontWeight: FontWeight.bold)),
            ),
            Text("${session.tempo.floor()}bpm"),
          ])
        ]));
  }
}
