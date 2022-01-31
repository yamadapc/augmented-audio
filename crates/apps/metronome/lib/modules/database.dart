import 'dart:async';

import 'package:floor/floor.dart';
import 'package:sqflite/sqflite.dart' as sqflite;

import 'history/session_dao.dart';
import 'history/session_entity.dart';

part 'database.g.dart';

@Database(version: 3, entities: [Session], views: [AggregatedSession])
abstract class MetronomeDatabase extends FloorDatabase {
  SessionDao get sessionDao;
}

final addBeatsPerBar = Migration(2, 3, (database) async {
  await database
      .execute("ALTER TABLE session ADD COLUMN beatsPerBar int DEFAULT 4");
});

final addAggregatedSession = Migration(3, 4, (database) async {
  await database.execute("""
DROP VIEW IF EXISTS AggregatedSession;
CREATE VIEW IF NOT EXISTS AggregatedSession AS
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
 """);
});

Future<MetronomeDatabase> buildDatabase() {
  return $FloorMetronomeDatabase
      .databaseBuilder('metronome_database.db')
      .addMigrations([addBeatsPerBar, addAggregatedSession]).build();
}
