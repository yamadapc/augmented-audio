import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/ui/controls/beats_per_bar_control.dart';
import 'package:metronome/ui/tabs/history/history_chart.dart';
import 'package:metronome/ui/tabs/history/history_list_item.dart';

class HistoryPageTab extends StatefulWidget {
  final HistoryStateController stateController;

  const HistoryPageTab({Key? key, required this.stateController})
      : super(key: key);

  @override
  State<HistoryPageTab> createState() => _HistoryPageTabState();
}

class _HistoryPageTabState extends State<HistoryPageTab> {
  ScrollController scrollController = ScrollController();

  @override
  void initState() {
    widget.stateController.refresh();
    super.initState();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    fireViewedAnalytics();
  }

  @override
  void activate() {
    super.activate();
    fireViewedAnalytics();
  }

  void fireViewedAnalytics() {
    final analytics = AppContext.of(context).analytics;
    analytics.logScreenView(
      screenClass: "HistoryPageTab",
      screenName: "History Page",
    );
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
                historyStateModel: widget.stateController.model,
              ),
            ),
            HistoryResolutionControl(
              historyStateController: widget.stateController,
            ),
            // const Divider(),
            // GoalPanel(),
            const Divider(),
            Expanded(
              child: ListView.builder(
                controller: scrollController,
                itemCount: widget.stateController.model.sessions.length,
                itemBuilder: (context, index) => HistoryListItem(
                  session: widget.stateController.model.sessions[index],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class HistoryResolutionControl extends StatelessWidget {
  final HistoryStateController historyStateController;

  const HistoryResolutionControl({
    Key? key,
    required this.historyStateController,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) => Container(
        width: double.infinity,
        padding: const EdgeInsets.only(left: 8.0, right: 8.0, top: 8.0),
        child: CupertinoSegmentedControl(
          groupValue: historyStateController.model.historyResolution,
          children: {
            HistoryResolution.weeks: segmentedControlText("Weeks"),
            HistoryResolution.days: segmentedControlText("Days"),
          },
          padding: EdgeInsets.zero,
          onValueChanged: (HistoryResolution value) {
            historyStateController.model.historyResolution = value;
          },
        ),
      ),
    );
  }
}
