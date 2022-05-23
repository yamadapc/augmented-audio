import 'package:floor/floor.dart';

@entity
class Session {
  @PrimaryKey(autoGenerate: true)
  final int? id;

  final int timestampMs;

  final int durationMs;

  final double tempo;

  final int beatsPerBar;

  Session(
      this.id, this.timestampMs, this.durationMs, this.tempo, this.beatsPerBar);

  Session.create(
      {this.id,
      required this.timestampMs,
      required this.durationMs,
      required this.tempo,
      required this.beatsPerBar});
}

@DatabaseView("""
SELECT
  SUM(durationMs) as durationMs,
  ((timestampMs / (1000 * 60 * 60 * 24)) * (1000 * 60 * 60 * 24)) as timestampMs,
  tempo,
  beatsPerBar
FROM session
GROUP BY
  ((timestampMs / (1000 * 60 * 60 * 24)) * (1000 * 60 * 60 * 24)),
  tempo,
  beatsPerBar
ORDER BY timestampMs DESC
  """, viewName: "AggregatedSession")
class AggregatedSession {
  final int durationMs;
  final int timestampMs;
  final double tempo;
  final int beatsPerBar;

  AggregatedSession(
      this.durationMs, this.timestampMs, this.tempo, this.beatsPerBar);
}

@DatabaseView("""
  SELECT
      SUM(durationMs) as durationMs,
      strftime('%s', datetime(timestampMs / 1000, 'unixepoch', 'localtime', 'start of day')) * 1000 AS timestampMs
  FROM session
  GROUP BY
      datetime(timestampMs / 1000, 'unixepoch', 'localtime', 'start of day')
  ORDER BY timestampMs DESC
""", viewName: "dailypracticetime")
class DailyPracticeTime {
  final int durationMs;
  final int timestampMs;

  DailyPracticeTime(
      this.durationMs, this.timestampMs);
}
