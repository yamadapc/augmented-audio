import 'package:floor/floor.dart';

@entity
class Session {
  @PrimaryKey(autoGenerate: true)
  final int? id;

  final int timestampMs;

  final int durationMs;

  final double tempo;

  Session(this.id, this.timestampMs, this.durationMs, this.tempo);
}
