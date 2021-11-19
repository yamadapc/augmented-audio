import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';

class PanelTab {
  final int id;
  final String title;
  final Widget view;

  PanelTab(this.id, this.title, this.view);
}

class PanelTabsView extends StatefulWidget {
  final List<PanelTab> tabs;

  const PanelTabsView({Key? key, required this.tabs}) : super(key: key);

  @override
  State<PanelTabsView> createState() => _PanelTabsViewState();
}

class _PanelTabsViewState extends State<PanelTabsView> {
  int selectedTab = 0;

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      Container(
          decoration: const BoxDecoration(
              border: Border(
                  bottom: BorderSide(color: Color.fromRGBO(0, 0, 0, 1.0))),
              color: Color.fromRGBO(80, 80, 80, 1.0)),
          child: DawTextStyle(
              child: Row(
                  children: widget.tabs
                      .map((tab) => Container(
                            decoration: BoxDecoration(
                                color: tab.id == selectedTab
                                    ? const Color.fromRGBO(20, 20, 20, 1.0)
                                    : Colors.transparent),
                            child: TextButton(
                                style: ButtonStyle(
                                    foregroundColor:
                                        MaterialStateProperty.all(Colors.white),
                                    backgroundColor: MaterialStateProperty.all(
                                        Colors.transparent),
                                    alignment: Alignment.centerLeft,
                                    textStyle: MaterialStateProperty.all(
                                        const TextStyle(color: Colors.white))),
                                onPressed: () {
                                  onTabPressed(tab);
                                },
                                child: Text(tab.title)),
                          ))
                      .toList()))),
      Expanded(child: widget.tabs[selectedTab].view)
    ]);
  }

  void onTabPressed(PanelTab tab) {
    setState(() {
      selectedTab = tab.id;
    });
  }
}
