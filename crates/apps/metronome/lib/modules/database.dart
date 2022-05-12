import 'dart:async';

import 'package:floor/floor.dart';
import 'package:sqflite/sqflite.dart' as sqflite;

import '../logger.dart';
import 'history/session_dao.dart';
import 'history/session_entity.dart';

part 'database.g.dart';

@Database(version: 4, entities: [Session], views: [AggregatedSession])
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

final migrations = [addBeatsPerBar, addAggregatedSession];

Future<MetronomeDatabase> buildInMemoryDatabase() {
  return $FloorMetronomeDatabase
      .inMemoryDatabaseBuilder()
      .addMigrations(migrations)
      .build();
}

const databaseName = 'metronome_database.db';

Future<MetronomeDatabase> buildDatabase() async {
  var path = await sqfliteDatabaseFactory.getDatabasePath(databaseName);
  logger.i("Opening SQLite database path=$path");
  MetronomeDatabase db = await $FloorMetronomeDatabase
      .databaseBuilder(databaseName)
      .addMigrations(migrations)
      .build();
  return db;
}
