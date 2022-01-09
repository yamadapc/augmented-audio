import 'package:floor/floor.dart';

import 'history/session_dao.dart';
import 'history/session_entity.dart';

part 'database.g.dart';

@Database(version: 1, entities: [Session])
abstract class MetronomeDatabase extends FloorDatabase {
  SessionDao get sessionDao;
}
