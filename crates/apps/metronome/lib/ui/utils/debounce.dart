import 'dart:async';

import 'package:flutter/cupertino.dart';

class Debounce {
  int _debounceMs;
  Timer? _timer;
  VoidCallback? _callback;

  Debounce(this._debounceMs);

  setDebounceMs(int value) {
    _debounceMs = value;
  }

  run(VoidCallback callback) {
    _timer?.cancel();
    _callback = callback;
    _timer = Timer(Duration(milliseconds: _debounceMs), () {
      _timer = null;
      _callback = null;

      callback();
    });
  }

  flush() {
    _timer?.cancel();
    _timer = null;
    if (_callback != null) {
      _callback!();
      _callback = null;
    }
  }

  cancel() {
    _timer?.cancel();
    _timer = null;
    _callback = null;
  }
}
