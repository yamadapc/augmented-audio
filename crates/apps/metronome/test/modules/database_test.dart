import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/database.dart';

void main() {
  test("We can open a database locally", () async {
    final database = await buildDatabase();
    await database.close();
  });
}
