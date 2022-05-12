import 'package:floor/floor.dart';

import 'session_entity.dart';

@dao
abstract class SessionDao {
  @Query("SELECT * FROM session ORDER BY timestampMs DESC LIMIT 100")
  Future<List<Session>> findAllSessions();

  @Query(
      "SELECT * FROM aggregatedsession WHERE timestampMs >= :startMs ORDER BY timestampMs DESC LIMIT 100")
  Future<List<AggregatedSession>> findAggregatedSessions(int startMs);

  @Update(onConflict: OnConflictStrategy.replace)
  Future<void> updateSession(Session session);

  @insert
  Future<int> insertSession(Session session);
}
