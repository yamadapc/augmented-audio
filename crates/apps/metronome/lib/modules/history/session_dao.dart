import 'package:floor/floor.dart';

import 'session_entity.dart';

@dao
abstract class SessionDao {
  @Query("SELECT * FROM Session ORDER BY timestampMs DESC LIMIT 100")
  Future<List<Session>> findAllSessions();

  @Query("SELECT * FROM AggregatedSession ORDER BY timestampMs DESC LIMIT 100")
  Future<List<AggregatedSession>> findAggregatedSessions();

  @Update(onConflict: OnConflictStrategy.replace)
  Future<void> updateSession(Session session);

  @insert
  Future<int> insertSession(Session session);
}
