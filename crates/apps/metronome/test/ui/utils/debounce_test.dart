import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/ui/utils/debounce.dart';

void main() {
  test("debounce will flush only once after duration", () async {
    var counter = 0;
    final debounced = Debounce(100);
    for (var i = 0; i < 10; i++) {
      debounced.run(() {
        counter++;
      });
    }
    expect(counter, equals(0));
    await Future.delayed(const Duration(milliseconds: 150));
    expect(counter, equals(1));
  });

  test("we can force flush", () {
    var counter = 0;
    final debounced = Debounce(100);
    for (var i = 0; i < 10; i++) {
      debounced.run(() {
        counter++;
      });
    }
    expect(counter, equals(0));
    debounced.flush();
    expect(counter, equals(1));
  });

  test("we can cancel pending operations", () {
    var counter = 0;
    final debounced = Debounce(100);
    for (var i = 0; i < 10; i++) {
      debounced.run(() {
        counter++;
      });
    }
    expect(counter, equals(0));
    debounced.cancel();
    debounced.flush();
    expect(counter, equals(0));
  });
}
