// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'database.dart';

// **************************************************************************
// FloorGenerator
// **************************************************************************

abstract class $MetronomeDatabaseBuilderContract {
  /// Adds migrations to the builder.
  $MetronomeDatabaseBuilderContract addMigrations(List<Migration> migrations);

  /// Adds a database [Callback] to the builder.
  $MetronomeDatabaseBuilderContract addCallback(Callback callback);

  /// Creates the database and initializes it.
  Future<MetronomeDatabase> build();
}

// ignore: avoid_classes_with_only_static_members
class $FloorMetronomeDatabase {
  /// Creates a database builder for a persistent database.
  /// Once a database is built, you should keep a reference to it and re-use it.
  static $MetronomeDatabaseBuilderContract databaseBuilder(String name) =>
      _$MetronomeDatabaseBuilder(name);

  /// Creates a database builder for an in memory database.
  /// Information stored in an in memory database disappears when the process is killed.
  /// Once a database is built, you should keep a reference to it and re-use it.
  static $MetronomeDatabaseBuilderContract inMemoryDatabaseBuilder() =>
      _$MetronomeDatabaseBuilder(null);
}

class _$MetronomeDatabaseBuilder implements $MetronomeDatabaseBuilderContract {
  _$MetronomeDatabaseBuilder(this.name);

  final String? name;

  final List<Migration> _migrations = [];

  Callback? _callback;

  @override
  $MetronomeDatabaseBuilderContract addMigrations(List<Migration> migrations) {
    _migrations.addAll(migrations);
    return this;
  }

  @override
  $MetronomeDatabaseBuilderContract addCallback(Callback callback) {
    _callback = callback;
    return this;
  }

  @override
  Future<MetronomeDatabase> build() async {
    final path = name != null
        ? await sqfliteDatabaseFactory.getDatabasePath(name!)
        : ':memory:';
    final database = _$MetronomeDatabase();
    database.database = await database.open(
      path,
      _migrations,
      _callback,
    );
    return database;
  }
}

class _$MetronomeDatabase extends MetronomeDatabase {
  _$MetronomeDatabase([StreamController<String>? listener]) {
    changeListener = listener ?? StreamController<String>.broadcast();
  }

  SessionDao? _sessionDaoInstance;

  Future<sqflite.Database> open(
    String path,
    List<Migration> migrations, [
    Callback? callback,
  ]) async {
    final databaseOptions = sqflite.OpenDatabaseOptions(
      version: 6,
      onConfigure: (database) async {
        await database.execute('PRAGMA foreign_keys = ON');
        await callback?.onConfigure?.call(database);
      },
      onOpen: (database) async {
        await callback?.onOpen?.call(database);
      },
      onUpgrade: (database, startVersion, endVersion) async {
        await MigrationAdapter.runMigrations(
            database, startVersion, endVersion, migrations);

        await callback?.onUpgrade?.call(database, startVersion, endVersion);
      },
      onCreate: (database, version) async {
        await database.execute(
            'CREATE TABLE IF NOT EXISTS `Session` (`id` INTEGER PRIMARY KEY AUTOINCREMENT, `timestampMs` INTEGER NOT NULL, `durationMs` INTEGER NOT NULL, `tempo` REAL NOT NULL, `beatsPerBar` INTEGER NOT NULL)');

        await database.execute(
            'CREATE VIEW IF NOT EXISTS `AggregatedSession` AS SELECT\n  SUM(durationMs) as durationMs,\n  ((timestampMs / (1000 * 60 * 60 * 24)) * (1000 * 60 * 60 * 24)) as timestampMs,\n  MIN(timestampMs) as startTimestampMs,\n  tempo,\n  beatsPerBar\nFROM session\nGROUP BY\n  ((timestampMs / (1000 * 60 * 60 * 24)) * (1000 * 60 * 60 * 24)),\n  tempo,\n  beatsPerBar\nORDER BY timestampMs DESC\n  ');
        await database.execute(
            'CREATE VIEW IF NOT EXISTS `dailypracticetime` AS   SELECT\n      SUM(durationMs) as durationMs,\n      strftime(\'%s\', datetime(timestampMs / 1000, \'unixepoch\', \'localtime\', \'start of day\')) * 1000 AS timestampMs\n  FROM session\n  GROUP BY\n      datetime(timestampMs / 1000, \'unixepoch\', \'localtime\', \'start of day\')\n  ORDER BY timestampMs DESC\n');

        await callback?.onCreate?.call(database, version);
      },
    );
    return sqfliteDatabaseFactory.openDatabase(path, options: databaseOptions);
  }

  @override
  SessionDao get sessionDao {
    return _sessionDaoInstance ??= _$SessionDao(database, changeListener);
  }
}

class _$SessionDao extends SessionDao {
  _$SessionDao(
    this.database,
    this.changeListener,
  )   : _queryAdapter = QueryAdapter(database),
        _sessionInsertionAdapter = InsertionAdapter(
            database,
            'Session',
            (Session item) => <String, Object?>{
                  'id': item.id,
                  'timestampMs': item.timestampMs,
                  'durationMs': item.durationMs,
                  'tempo': item.tempo,
                  'beatsPerBar': item.beatsPerBar
                }),
        _sessionUpdateAdapter = UpdateAdapter(
            database,
            'Session',
            ['id'],
            (Session item) => <String, Object?>{
                  'id': item.id,
                  'timestampMs': item.timestampMs,
                  'durationMs': item.durationMs,
                  'tempo': item.tempo,
                  'beatsPerBar': item.beatsPerBar
                });

  final sqflite.DatabaseExecutor database;

  final StreamController<String> changeListener;

  final QueryAdapter _queryAdapter;

  final InsertionAdapter<Session> _sessionInsertionAdapter;

  final UpdateAdapter<Session> _sessionUpdateAdapter;

  @override
  Future<List<Session>> findAllSessions() async {
    return _queryAdapter.queryList(
        'SELECT * FROM session ORDER BY timestampMs DESC LIMIT 100',
        mapper: (Map<String, Object?> row) => Session(
            row['id'] as int?,
            row['timestampMs'] as int,
            row['durationMs'] as int,
            row['tempo'] as double,
            row['beatsPerBar'] as int));
  }

  @override
  Future<List<AggregatedSession>> findAggregatedSessions() async {
    return _queryAdapter.queryList(
        'SELECT * FROM aggregatedsession ORDER BY startTimestampMs DESC LIMIT 100',
        mapper: (Map<String, Object?> row) => AggregatedSession(
            row['durationMs'] as int,
            row['timestampMs'] as int,
            row['startTimestampMs'] as int,
            row['tempo'] as double,
            row['beatsPerBar'] as int));
  }

  @override
  Future<List<DailyPracticeTime>> findDailyPracticeTime(int startMs) async {
    return _queryAdapter.queryList(
        'SELECT * FROM dailypracticetime WHERE timestampMs >= ?1 ORDER BY timestampMs DESC LIMIT 100',
        mapper: (Map<String, Object?> row) => DailyPracticeTime(row['durationMs'] as int, row['timestampMs'] as int),
        arguments: [startMs]);
  }

  @override
  Future<int> insertSession(Session session) {
    return _sessionInsertionAdapter.insertAndReturnId(
        session, OnConflictStrategy.abort);
  }

  @override
  Future<void> updateSession(Session session) async {
    await _sessionUpdateAdapter.update(session, OnConflictStrategy.replace);
  }
}
