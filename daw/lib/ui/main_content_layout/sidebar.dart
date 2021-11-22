import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/ui_state.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_daw_mock_ui/ui/common/tabs.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'sidebar/sidebar_browser.dart';

class Sidebar extends StatelessWidget {
  final SidebarState sidebarState;

  const Sidebar({Key? key, required this.sidebarState}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var tabs = [
      ConcretePanelTab(
          0,
          "Browser",
          Container(
              decoration:
                  const BoxDecoration(color: Color.fromRGBO(50, 50, 50, 1.0)),
              child: const SidebarBrowser()))
    ];
    return Observer(
      builder: (_) => DawTextStyle(
        child: sidebarState.panelState.isExpanded
            ? SizedBox(
                width: sidebarState.panelState.size,
                child: PanelTabsView(
                  showVerticalTabs: true,
                  onMinimize: onMinimize,
                  tabs: tabs,
                ),
              )
            : Container(
                decoration: const BoxDecoration(
                    border: Border(
                        right: BorderSide(color: Color.fromRGBO(0, 0, 0, 1.0))),
                    color: Color.fromRGBO(80, 80, 80, 1.0)),
                child: RotatedBox(
                  quarterTurns: -1,
                  child: Row(
                    children: [
                      const Spacer(),
                      SelectableButton(
                          onPressed: () {
                            onMinimize();
                          },
                          isSelected: true,
                          child: const Text("Browser"))
                    ],
                  ),
                )),
      ),
    );
  }

  void onMinimize() {
    sidebarState.panelState.toggleExpanded();
  }
}
