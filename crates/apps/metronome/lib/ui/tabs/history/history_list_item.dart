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
  final hours = duration.inHours;
  final minutes = duration.inMinutes - hours * 60;
  final seconds = duration.inSeconds - hours * (60 * 60) - minutes * 60;

  if (hours == 0 && minutes == 0) {
    return formatDurationNumber("s", seconds);
  } else if (hours == 0) {
    final fminutes = formatDurationNumber("m", minutes);
    final fseconds = formatDurationNumber("s", seconds);
    return [fminutes, fseconds].join();
  }

  final fhours = formatDurationNumber("h", hours);
  final fminutes = formatDurationNumber("m", minutes);
  final fseconds = formatDurationNumber("s", seconds);
  return [fhours, fminutes, fseconds].join();
}

class HistoryListItem extends StatelessWidget {
  final AggregatedSession session;

  const HistoryListItem({super.key, required this.session});

  @override
  Widget build(BuildContext context) {
    final DateFormat dateFormat = DateFormat("yyyy-MM-dd • hh:mm");
    final formattedDate = dateFormat
        .format(DateTime.fromMillisecondsSinceEpoch(session.startTimestampMs));
    final duration = Duration(milliseconds: session.durationMs);
    final timeSignature = "${session.beatsPerBar}/4";

    return Container(
      padding: const EdgeInsets.only(left: 8.0, right: 8.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(formattedDate, style: const TextStyle(fontSize: 14)),
          Row(
            children: [
              Expanded(
                child: Text(
                  formatDuration(duration),
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              Text("$timeSignature - ${session.tempo.floor()}bpm"),
            ],
          ),
          const Divider(),
        ],
      ),
    );
  }
}
