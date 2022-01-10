import 'dart:async';

import 'package:floor/floor.dart';
import 'package:sqflite/sqflite.dart' as sqflite;

import 'history/session_dao.dart';
import 'history/session_entity.dart';

part 'database.g.dart';

@Database(version: 2, entities: [Session])
abstract class MetronomeDatabase extends FloorDatabase {
  SessionDao get sessionDao;
}
