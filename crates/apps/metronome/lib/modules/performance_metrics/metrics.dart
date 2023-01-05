/// This file provides a very lousy facade for performance metrics
/// instrumentation following more or less the same API as firebase performance
/// traces which aren't available on macOS yet.
import 'package:clock/clock.dart';
import 'package:metronome/logger.dart';

final _metrics = _PerformanceMetricsImpl();

PerformanceMetrics getMetrics() {
  return _metrics;
}

abstract class PerformanceMetrics {
  PerformanceTrace newTrace(String name);
}

abstract class PerformanceTrace {
  void start();
  void stop();
  void setMetric(String name, dynamic value);
}

class _PerformanceMetricsImpl implements PerformanceMetrics {
  @override
  PerformanceTrace newTrace(String name) {
    return _PerformanceTraceImpl(name: name);
  }
}

class _PerformanceTraceImpl implements PerformanceTrace {
  final String name;
  DateTime _startTime = clock.now();
  final Map<String, dynamic> _attributes = {};

  _PerformanceTraceImpl({required this.name});

  @override
  void setMetric(String name, dynamic value) {
    _attributes[name] = value;
  }

  @override
  void start() {
    _startTime = clock.now();
  }

  @override
  void stop() {
    final duration = clock.now().difference(_startTime);
    logger
        .i('Trace "$name" finished duration=$duration attributes=$_attributes');
  }
}
