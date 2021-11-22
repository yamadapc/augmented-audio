import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/ui_state.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

class ConcretePanelTab {
  final int id;
  final String title;
  final Widget view;

  ConcretePanelTab(this.id, this.title, this.view);
}

class PanelTabsView extends StatelessWidget {
  final List<ConcretePanelTab> tabs;
  final void Function()? onMinimize;
  late final PanelTabsState panelTabsState;
  late final bool showVerticalTabs;

  PanelTabsView(
      {Key? key,
      required this.tabs,
      this.onMinimize,
      PanelTabsState? panelTabsState,
      bool? showVerticalTabs})
      : super(key: key) {
    this.panelTabsState = panelTabsState ?? PanelTabsState();
    this.showVerticalTabs = showVerticalTabs ?? false;
  }

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        var extraItems = [];
        if (showVerticalTabs) {
          var verticalTabs = Container(
              decoration: const BoxDecoration(
                  border: Border(
                      right: BorderSide(color: Color.fromRGBO(0, 0, 0, 1.0))),
                  color: Color.fromRGBO(80, 80, 80, 1.0)),
              child: RotatedBox(
                quarterTurns: -1,
                child: Row(
                  children: [
                    const Spacer(),
                    ...tabs.map((tab) {
                      return SelectableButton(
                          onPressed: () {
                            onMinimize!();
                          },
                          isSelected: panelTabsState.selectedIndex == tab.id,
                          child: Text(tab.title));
                    }).toList(),
                  ],
                ),
              ));
          extraItems.add(verticalTabs);
        }

        return Container(
          decoration: const BoxDecoration(
            border:
                Border(right: BorderSide(color: Color.fromRGBO(0, 0, 0, 1.0))),
          ),
          child: Row(
            children: [
              ...extraItems,
              Expanded(
                child: Column(children: [
                  Container(
                      decoration: const BoxDecoration(
                          border: Border(
                              bottom: BorderSide(
                                  color: Color.fromRGBO(0, 0, 0, 1.0))),
                          color: Color.fromRGBO(80, 80, 80, 1.0)),
                      child: PanelTabsHeaderView(
                          hasVerticalTabs: showVerticalTabs,
                          tabs: tabs,
                          panelTabsState: panelTabsState,
                          onMinimize: onMinimize)),
                  Expanded(child: tabs[panelTabsState.selectedIndex].view)
                ]),
              ),
            ],
          ),
        );
      },
    );
  }
}

ButtonStyle textButtonStyle(BuildContext context) => ButtonStyle(
    foregroundColor: MaterialStateProperty.all(Colors.white),
    backgroundColor: MaterialStateProperty.all(Colors.transparent),
    alignment: Alignment.centerLeft,
    textStyle: MaterialStateProperty.all(DefaultTextStyle.of(context)
        .style
        .merge(const TextStyle(color: Colors.white))));

class PanelTabsHeaderView extends StatelessWidget {
  final PanelTabsState panelTabsState;
  final List<ConcretePanelTab> tabs;
  final void Function()? onMinimize;
  late final bool hasVerticalTabs;

  PanelTabsHeaderView({
    Key? key,
    required this.tabs,
    required this.panelTabsState,
    this.onMinimize,
    bool? hasVerticalTabs,
  }) : super(key: key) {
    this.hasVerticalTabs = hasVerticalTabs ?? false;
  }

  @override
  Widget build(BuildContext context) {
    Widget leftHandContent = hasVerticalTabs
        ? Padding(
            padding: const EdgeInsets.all(8.0),
            child: Text(tabs[panelTabsState.selectedIndex].title),
          )
        : Row(
            children: tabs
                .map((tab) => SelectableButton(
                      isSelected: tab.id == panelTabsState.selectedIndex,
                      onPressed: () {
                        panelTabsState.setSelectedIndex(tab.id);
                      },
                      child: Text(tab.title),
                    ))
                .toList());

    var rightHandButtons = [];
    if (onMinimize != null) {
      rightHandButtons.add(SizedBox(
        height: 20,
        child: IconButton(
          alignment: Alignment.center,
          padding: const EdgeInsets.all(0),
          icon: Stack(
            children: const [
              Positioned(
                left: 8,
                bottom: 5,
                child: Icon(Icons.minimize),
              ),
            ],
          ),
          onPressed: () {
            onMinimize!();
          },
        ),
      ));
    }

    return DawTextStyle(
        child: SizedBox(
      height: 28,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [Expanded(child: leftHandContent), ...rightHandButtons],
      ),
    ));
  }
}

class SelectableButton extends StatelessWidget {
  final bool isSelected;
  final Widget child;
  final void Function() onPressed;

  const SelectableButton(
      {Key? key,
      required this.isSelected,
      required this.onPressed,
      required this.child})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
          color: isSelected
              ? const Color.fromRGBO(20, 20, 20, 1.0)
              : Colors.transparent),
      child: TextButton(
          style: textButtonStyle(context), onPressed: onPressed, child: child),
    );
  }
}
