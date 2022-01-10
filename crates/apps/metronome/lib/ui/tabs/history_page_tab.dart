import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:metronome/modules/state/history_state_controller.dart';

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
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => SafeArea(
        child: ListView(
            children: widget.stateController.model.sessions
                .map((element) => HistoryListItem(session: element))
                .toList()),
      ),
    );
  }
}
