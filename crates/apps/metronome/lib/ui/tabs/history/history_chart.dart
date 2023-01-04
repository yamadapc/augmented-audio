import 'package:charts_flutter/flutter.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:intl/intl.dart';
import 'package:metronome/modules/state/history_state_model.dart';

class HistoryChart extends StatelessWidget {
  final HistoryStateModel historyStateModel;

  const HistoryChart({Key? key, required this.historyStateModel})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        final DateFormat dateFormat =
            historyStateModel.historyResolution == HistoryResolution.days
                ? DateFormat("E")
                : DateFormat("d/MM/yy");
        final List<MapEntry<int, int>> data = getData();

        final seriesList = [
          Series<MapEntry<int, int>, String>(
            id: "Practice Sessions",
            data: data,
            domainFn: (MapEntry<int, int> entry, _) {
              final formattedDate = dateFormat
                  .format(DateTime.fromMillisecondsSinceEpoch(entry.key));
              return formattedDate;
            },
            measureFn: (MapEntry<int, int> entry, _) => entry.value,
          )
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
            renderSpec:
                SmallTickRendererSpec(labelStyle: TextStyleSpec(fontSize: 10)),
          ),
          layoutConfig: LayoutConfig(
            leftMarginSpec: MarginSpec.fixedPixel(0),
            bottomMarginSpec: MarginSpec.fixedPixel(20),
            rightMarginSpec: MarginSpec.fixedPixel(0),
            topMarginSpec: MarginSpec.fixedPixel(0),
          ),
          defaultRenderer: BarRendererConfig(minBarLengthPx: 100),
        );
      },
    );
  }

  List<MapEntry<int, int>> getData() {
    final resolution = historyStateModel.historyResolution;

    if (resolution == HistoryResolution.weeks) {
      return historyStateModel.weeklyPracticeTime
          .map(
            (practiceTime) =>
                MapEntry(practiceTime.timestampMs, practiceTime.durationMs),
          )
          .toList();
    }

    return historyStateModel.dailyPracticeTime
        .map(
          (practiceTime) =>
              MapEntry(practiceTime.timestampMs, practiceTime.durationMs),
        )
        .toList();
  }
}
