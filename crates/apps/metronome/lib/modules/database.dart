import 'dart:async';

import 'package:floor/floor.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/history/session_dao.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:sqflite/sqflite.dart' as sqflite;

part 'database.g.dart';

@Database(
  version: 6,
  entities: [Session],
  views: [AggregatedSession, DailyPracticeTime],
)
abstract class MetronomeDatabase extends FloorDatabase {
  SessionDao get sessionDao;
}

final addBeatsPerBar = Migration(2, 3, (database) async {
  logger.i("Add beats per bar column");
  await database
      .execute("ALTER TABLE session ADD COLUMN beatsPerBar int DEFAULT 4");
});

final addAggregatedSession = Migration(3, 4, (database) async {
  logger.i("Migrating aggregated sessions view");
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

final addDailyPracticeTime = Migration(4, 5, (database) async {
  logger.i("Migrating daily session time");
  await database.execute("""
DROP VIEW IF EXISTS DailyPracticeTime;
CREATE VIEW IF NOT EXISTS DailyPracticeTime AS
  SELECT
      SUM(durationMs) as durationMs,
      strftime('%s', datetime(timestampMs / 1000, 'unixepoch', 'localtime', 'start of day')) * 1000 AS timestampMs
  FROM session
  GROUP BY
      datetime(timestampMs / 1000, 'unixepoch', 'localtime', 'start of day')
  ORDER BY timestampMs DESC 
  """);
});

final addStartAggregatedSessionTime = Migration(5, 6, (database) async {
  logger.i("Migrating aggregated sessions view");
  await database.execute("""
DROP VIEW IF EXISTS AggregatedSession;
CREATE VIEW IF NOT EXISTS AggregatedSession AS
  SELECT
    SUM(durationMs) as durationMs,
    ((timestampMs / (1000 * 60 * 60 * 24)) * (1000 * 60 * 60 * 24)) as timestampMs,
    MIN(timestampMs) as startTimestampMs,
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

final migrations = [
  addBeatsPerBar,
  addAggregatedSession,
  addDailyPracticeTime,
  addStartAggregatedSessionTime,
];

Future<MetronomeDatabase> buildInMemoryDatabase() {
  return $FloorMetronomeDatabase
      .inMemoryDatabaseBuilder()
      .addMigrations(migrations)
      .build();
}

const databaseName = 'metronome_database.db';

Future<MetronomeDatabase> buildDatabase() async {
  final path = await sqfliteDatabaseFactory.getDatabasePath(databaseName);
  logger.i("Opening SQLite database path=$path");
  final MetronomeDatabase db = await $FloorMetronomeDatabase
      .databaseBuilder(databaseName)
      .addMigrations(migrations)
      .build();
  return db;
}
