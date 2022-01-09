import 'package:floor/floor.dart';

import 'session_entity.dart';

@dao
abstract class SessionDao {
  @Query("SELECT * FROM Session")
  Future<List<Session>> findAllSessions();

  @insert
  Future<void> insertSession(Session session);
}
