import 'package:charts_flutter/flutter.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:mobx/mobx.dart';

part 'history_state_model.g.dart';

// ignore: library_private_types_in_public_api
class HistoryStateModel = _HistoryStateModel with _$HistoryStateModel;

enum HistoryResolution {
  weeks,
  days,
}

abstract class _HistoryStateModel with Store {
  @observable
  ObservableList<AggregatedSession> sessions = ObservableList.of([]);

  @observable
  ObservableList<DailyPracticeTime> dailyPracticeTime = ObservableList.of([]);

  @observable
  ObservableList<DailyPracticeTime> weeklyPracticeTime = ObservableList.of([]);

  @observable
  HistoryResolution historyResolution = HistoryResolution.days;

  @computed
  HistoryChartModel get chartModel {
    final data = historyResolution == HistoryResolution.days
        ? dailyPracticeTime
        : weeklyPracticeTime;
    final DateFormat dateFormat = historyResolution == HistoryResolution.days
        ? DateFormat("E")
        : DateFormat("d/MM/yy");
    final points = ChartTransformer.preprocessPoints(
      DateTime.now(),
      data,
      historyResolution,
    );

    return HistoryChartModel(data: points, dateFormat: dateFormat);
  }
}

mixin ChartTransformer {
  @visibleForTesting
  static DateTime startOfDay(DateTime now) {
    final result = DateTime(now.year, now.month, now.day);
    return result;
  }

  static List<DailyPracticeTime> preprocessPoints(
    DateTime now,
    List<DailyPracticeTime> data,
    HistoryResolution historyResolution,
  ) {
    final DateTime day = startOfDay(now);
    final List<DateTime> days = [];
    for (int i = 1; i <= 7; i++) {
      final DateTime d = startOfDay(day.subtract(Duration(days: 7 - i)));
      days.add(d);
    }
    final Map<DateTime, DailyPracticeTime> pointsByDay = getPointsByDay(data);

    return days
        .map(
          (d) => DailyPracticeTime.from(
            durationMs: pointsByDay[d]?.durationMs ?? 0,
            timestampMs: d.millisecondsSinceEpoch,
          ),
        )
        .toList();
  }

  static Map<DateTime, DailyPracticeTime> getPointsByDay(
    List<DailyPracticeTime> data,
  ) {
    final Map<DateTime, DailyPracticeTime> pointsByDay = {};
    for (final point in data) {
      final DateTime pointDate = DateTime.fromMillisecondsSinceEpoch(
        point.timestampMs,
      );
      pointsByDay[startOfDay(pointDate)] = point;
    }
    return pointsByDay;
  }
}

typedef HistoryChartPoint = MapEntry<int, int>;
typedef HistoryChartSeries = Series<HistoryChartPoint, String>;
typedef HistoryChartSeriesList = List<HistoryChartSeries>;

class HistoryChartModel {
  List<DailyPracticeTime> data;
  DateFormat dateFormat;

  HistoryChartModel({required this.data, required this.dateFormat});

  HistoryChartSeriesList get seriesList {
    return [
      HistoryChartSeries(
        id: "Practice Sessions",
        data: _getChartPoints(),
        domainFn: (HistoryChartPoint entry, _) {
          final formattedDate =
              dateFormat.format(DateTime.fromMillisecondsSinceEpoch(entry.key));
          return formattedDate;
        },
        measureFn: (HistoryChartPoint entry, _) => entry.value,
      )
    ];
  }

  List<HistoryChartPoint> _getChartPoints() {
    return data
        .map(
          (practiceTime) =>
              MapEntry(practiceTime.timestampMs, practiceTime.durationMs),
        )
        .toList();
  }
}
