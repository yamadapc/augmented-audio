import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/session_entity.dart';

void main() {
  test("SessionDao will start-off empty in tests", () async {
    final database = await buildInMemoryDatabase();
    final sessions = await database.sessionDao.findAllSessions();
    expect(sessions.length, equals(0));
  });

  test("SessionDao can insert sessions", () async {
    final database = await buildInMemoryDatabase();
    expect(await database.sessionDao.findAllSessions(), equals([]));
    final sessionId = await database.sessionDao.insertSession(Session.create(
        timestampMs: 0, durationMs: 100, tempo: 120, beatsPerBar: 4));

    final sessions = await database.sessionDao.findAllSessions();
    expect(sessions.length, equals(1));
    final session = sessions.firstWhere((element) => element.id == sessionId);
    expect(session.tempo, equals(120));
  });
}
