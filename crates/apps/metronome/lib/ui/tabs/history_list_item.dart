import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:metronome/modules/history/session_entity.dart';

String formatDurationNumber(String postfix, int value) {
  if (value == 0) {
    return "";
  } else {
    return "$value$postfix";
  }
}

String formatDuration(Duration duration) {
  var hours = duration.inHours;
  var minutes = duration.inMinutes - hours * 60;
  var seconds = duration.inSeconds - hours * (60 * 60) - minutes * 60;

  if (hours == 0 && minutes == 0) {
    return formatDurationNumber("s", seconds);
  } else if (hours == 0) {
    var fminutes = formatDurationNumber("m", minutes);
    var fseconds = formatDurationNumber("s", seconds);
    return [fminutes, fseconds].join("");
  }

  var fhours = formatDurationNumber("h", hours);
  var fminutes = formatDurationNumber("m", minutes);
  var fseconds = formatDurationNumber("s", seconds);
  return [fhours, fminutes, fseconds].join("");
}

class HistoryListItem extends StatelessWidget {
  final AggregatedSession session;

  const HistoryListItem({Key? key, required this.session}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    DateFormat dateFormat = DateFormat("yyyy-MM-dd");
    final formattedDate = dateFormat
        .format(DateTime.fromMillisecondsSinceEpoch(session.timestampMs));
    final duration = Duration(milliseconds: session.durationMs);
    final timeSignature = "${session.beatsPerBar}/4";

    return Container(
        padding: const EdgeInsets.all(8.0),
        decoration: const BoxDecoration(
          border: Border(
              bottom: BorderSide(color: CupertinoColors.opaqueSeparator)),
        ),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Text(formattedDate, style: const TextStyle(fontSize: 14)),
          Row(children: [
            Expanded(
              child: Text(formatDuration(duration),
                  style: const TextStyle(
                      fontSize: 20, fontWeight: FontWeight.bold)),
            ),
            Text("$timeSignature - ${session.tempo.floor()}bpm"),
          ])
        ]));
  }
}
