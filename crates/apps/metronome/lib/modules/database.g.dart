// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'database.dart';

// **************************************************************************
// FloorGenerator
// **************************************************************************

// ignore: avoid_classes_with_only_static_members
class $FloorMetronomeDatabase {
  /// Creates a database builder for a persistent database.
  /// Once a database is built, you should keep a reference to it and re-use it.
  static _$MetronomeDatabaseBuilder databaseBuilder(String name) =>
      _$MetronomeDatabaseBuilder(name);

  /// Creates a database builder for an in memory database.
  /// Information stored in an in memory database disappears when the process is killed.
  /// Once a database is built, you should keep a reference to it and re-use it.
  static _$MetronomeDatabaseBuilder inMemoryDatabaseBuilder() =>
      _$MetronomeDatabaseBuilder(null);
}

class _$MetronomeDatabaseBuilder {
  _$MetronomeDatabaseBuilder(this.name);

  final String? name;

  final List<Migration> _migrations = [];

  Callback? _callback;

  /// Adds migrations to the builder.
  _$MetronomeDatabaseBuilder addMigrations(List<Migration> migrations) {
    _migrations.addAll(migrations);
    return this;
  }

  /// Adds a database [Callback] to the builder.
  _$MetronomeDatabaseBuilder addCallback(Callback callback) {
    _callback = callback;
    return this;
  }

  /// Creates the database and initializes it.
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

  Future<sqflite.Database> open(String path, List<Migration> migrations,
      [Callback? callback]) async {
    final databaseOptions = sqflite.OpenDatabaseOptions(
      version: 1,
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
            'CREATE TABLE IF NOT EXISTS `Session` (`id` INTEGER NOT NULL, `timestampMs` INTEGER NOT NULL, `durationMs` INTEGER NOT NULL, PRIMARY KEY (`id`))');

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
  _$SessionDao(this.database, this.changeListener)
      : _queryAdapter = QueryAdapter(database),
        _sessionInsertionAdapter = InsertionAdapter(
            database,
            'Session',
                (Session item) =>
            <String, Object?>{
              'id': item.id,
              'timestampMs': item.timestampMs,
              'durationMs': item.durationMs
            });

  final sqflite.DatabaseExecutor database;

  final StreamController<String> changeListener;

  final QueryAdapter _queryAdapter;

  final InsertionAdapter<Session> _sessionInsertionAdapter;

  @override
  Future<List<Session>> findAllSessions() async {
    return _queryAdapter.queryList('SELECT * FROM Session',
        mapper: (Map<String, Object?> row) =>
            Session(row['id'] as int,
                row['timestampMs'] as int, row['durationMs'] as int));
  }

  @override
  Future<void> insertSession(Session session) async {
    await _sessionInsertionAdapter.insert(session, OnConflictStrategy.abort);
  }
}
