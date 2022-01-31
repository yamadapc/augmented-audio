import 'dart:async';

import 'package:flutter/cupertino.dart';

class Debounce {
  final int _debounceMs;
  Timer? _timer;
  VoidCallback? _callback;

  Debounce(this._debounceMs);

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
