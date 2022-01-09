import 'package:floor/floor.dart';

@entity
class Session {
  @primaryKey
  final int id;

  final int timestampMs;

  final int durationMs;

  Session(this.id, this.timestampMs, this.durationMs);
}
