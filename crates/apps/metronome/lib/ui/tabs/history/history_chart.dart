import 'package:charts_flutter/flutter.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:intl/intl.dart';

import '../../../modules/state/history_state_model.dart';

class HistoryChart extends StatelessWidget {
  final HistoryStateModel historyStateModel;

  const HistoryChart({Key? key, required this.historyStateModel})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(builder: (_) {
      DateFormat dateFormat = DateFormat("E");
      var sessions = historyStateModel.sessions;

      Map<int, int> timeByDay = {};
      DateTime now = DateTime.now();
      DateTime today = DateTime(now.year, now.month, now.day);
      for (var i = 0; i < 7; i++) {
        final day = today.subtract(Duration(days: i));
        timeByDay[day.millisecondsSinceEpoch] = 0;
      }

      for (var session in sessions) {
        final timestamp =
            DateTime.fromMillisecondsSinceEpoch(session.timestampMs);
        final day = DateTime(timestamp.year, timestamp.month, timestamp.day);
        timeByDay.update(day.millisecondsSinceEpoch,
            (value) => value + session.durationMs.toInt());
      }
      List<MapEntry<int, int>> data = timeByDay.entries.toList();
      data.sort((entry1, entry2) => entry1.key > entry2.key ? 1 : -1);

      var seriesList = [
        Series<MapEntry<int, int>, String>(
            id: "Practice Sessions",
            data: data,
            domainFn: (MapEntry entry, _) {
              final formattedDate = dateFormat
                  .format(DateTime.fromMillisecondsSinceEpoch(entry.key));
              return formattedDate;
            },
            measureFn: (MapEntry entry, _) => entry.value)
      ];

      return BarChart(
        seriesList,
        animate: false,
        primaryMeasureAxis: const NumericAxisSpec(
          renderSpec: NoneRenderSpec(),
        ),
        domainAxis: const OrdinalAxisSpec(
          showAxisLine: true,
          tickProviderSpec: BasicOrdinalTickProviderSpec(),
          renderSpec: SmallTickRendererSpec(),
        ),
        layoutConfig: LayoutConfig(
            leftMarginSpec: MarginSpec.fixedPixel(0),
            bottomMarginSpec: MarginSpec.fixedPixel(20),
            rightMarginSpec: MarginSpec.fixedPixel(0),
            topMarginSpec: MarginSpec.fixedPixel(0)),
        defaultRenderer: BarRendererConfig(minBarLengthPx: 100),
      );
    });
  }
}
