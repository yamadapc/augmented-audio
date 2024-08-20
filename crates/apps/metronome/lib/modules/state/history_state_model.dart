import 'package:community_charts_flutter/community_charts_flutter.dart';
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
        : DateFormat("d/MM");
    final points = ChartTransformer.preprocessPoints(
      DateTime.now(),
      data,
      historyResolution,
    );

    final totalDuration = points
        .map((e) => e.durationMs)
        .reduce((value, element) => value + element);

    return HistoryChartModel(
      data: points,
      dateFormat: dateFormat,
      isEmpty: totalDuration <= 0,
    );
  }
}

mixin ChartTransformer {
  static DateTime startOfDate({
    required DateTime date,
    HistoryResolution? resolution,
  }) {
    if (resolution == HistoryResolution.weeks) {
      final nowPrime = date.subtract(Duration(days: date.weekday - 1));
      final result = DateTime(nowPrime.year, nowPrime.month, nowPrime.day);
      return result;
    }

    final result = DateTime(date.year, date.month, date.day);
    return result;
  }

  static List<DailyPracticeTime> preprocessPoints(
    DateTime now,
    List<DailyPracticeTime> data,
    HistoryResolution historyResolution,
  ) {
    final DateTime day = startOfDate(
      date: now,
      resolution: historyResolution,
    );
    final List<DateTime> days = [];

    for (int i = 1; i <= 7; i++) {
      if (historyResolution == HistoryResolution.weeks) {
        final DateTime d = startOfDate(
          date: day.subtract(Duration(days: 49 - i * 7)),
          resolution: historyResolution,
        );
        days.add(d);
      } else {
        final DateTime d = startOfDate(
          date: day.subtract(Duration(days: 7 - i)),
          resolution: historyResolution,
        );
        days.add(d);
      }
    }
    final Map<DateTime, DailyPracticeTime> pointsByDay =
        getPointsByDay(data, historyResolution);

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
    HistoryResolution historyResolution,
  ) {
    final Map<DateTime, DailyPracticeTime> pointsByDay = {};
    for (final point in data) {
      final DateTime pointDate = DateTime.fromMillisecondsSinceEpoch(
        point.timestampMs,
      );
      pointsByDay[startOfDate(date: pointDate, resolution: historyResolution)] =
          point;
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
  bool isEmpty;

  HistoryChartModel({
    required this.data,
    required this.dateFormat,
    required this.isEmpty,
  });

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
