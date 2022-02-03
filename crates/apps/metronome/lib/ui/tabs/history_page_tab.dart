import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:metronome/modules/state/history_state_controller.dart';

import 'history_chart.dart';
import 'history_list_item.dart';

class HistoryPageTab extends StatefulWidget {
  final HistoryStateController stateController;

  const HistoryPageTab({Key? key, required this.stateController})
      : super(key: key);

  @override
  State<HistoryPageTab> createState() => _HistoryPageTabState();
}

class _HistoryPageTabState extends State<HistoryPageTab> {
  @override
  void initState() {
    widget.stateController.refresh();
    super.initState();
  }

  @override
  void activate() {
    super.activate();
    var analytics = FirebaseAnalytics.instance;
    analytics.logScreenView(
        screenClass: "HistoryPageTab", screenName: "History Page");
  }

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => SafeArea(
        child: Column(
          children: [
            SizedBox(
                height: 80,
                child: HistoryChart(
                    historyStateModel: widget.stateController.model)),
            const Divider(),
            Expanded(
              child: ListView(
                  children: widget.stateController.model.sessions
                      .map((element) => HistoryListItem(session: element))
                      .toList()),
            ),
          ],
        ),
      ),
    );
  }
}
