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
      DateFormat dateFormat = DateFormat("dd-MM");
      var sessions = historyStateModel.sessions;
      var timeByDay = {};
      for (var session in sessions) {
        if (timeByDay[session.timestampMs] == null) {
          timeByDay[session.timestampMs] = 0;
        }
        timeByDay[session.timestampMs] += session.durationMs;
      }
      var data = timeByDay.entries.toList().reversed.toList();

      var seriesList = [
        Series(
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
          showAxisLine: false,
          tickProviderSpec: BasicOrdinalTickProviderSpec(),
          renderSpec: SmallTickRendererSpec(),
        ),
        layoutConfig: LayoutConfig(
            leftMarginSpec: MarginSpec.fixedPixel(0),
            bottomMarginSpec: MarginSpec.fixedPixel(20),
            rightMarginSpec: MarginSpec.fixedPixel(0),
            topMarginSpec: MarginSpec.fixedPixel(0)),
      );
    });
  }
}
