import 'dart:async';

import 'package:flutter/cupertino.dart';

class Debounce {
  int debounceMs;
  Timer? _timer;
  VoidCallback? _callback;

  Debounce(this.debounceMs);

  void run(VoidCallback callback) {
    _timer?.cancel();
    _callback = callback;
    _timer = Timer(Duration(milliseconds: debounceMs), () {
      _timer = null;
      _callback = null;

      callback();
    });
  }

  void flush() {
    _timer?.cancel();
    _timer = null;
    if (_callback != null) {
      _callback!();
      _callback = null;
    }
  }

  void cancel() {
    _timer?.cancel();
    _timer = null;
    _callback = null;
  }
}
