import 'package:floor/floor.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:mockito/annotations.dart';

@GenerateNiceMocks(
  [MockSpec<SessionDao>()],
)
// ignore: unused_import, always_use_package_imports
import './session_dao.mocks.dart';

@dao
abstract class SessionDao {
  @Query("SELECT * FROM session ORDER BY timestampMs DESC LIMIT 100")
  Future<List<Session>> findAllSessions();

  @Query(
    "SELECT * FROM aggregatedsession ORDER BY startTimestampMs DESC LIMIT 100",
  )
  Future<List<AggregatedSession>> findAggregatedSessions();

  @Query(
    "SELECT * FROM dailypracticetime WHERE timestampMs >= :startMs ORDER BY timestampMs DESC LIMIT 100",
  )
  Future<List<DailyPracticeTime>> findDailyPracticeTime(int startMs);

  @Update(onConflict: OnConflictStrategy.replace)
  Future<void> updateSession(Session session);

  @insert
  Future<int> insertSession(Session session);
}
