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
        final HistoryChartModel chartModel = historyStateModel.chartModel;
        final HistoryChartSeriesList seriesList = chartModel.seriesList;

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
}
