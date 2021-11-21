import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/generic_sidebar.dart';

class Category implements SidebarItem<Category> {
  @override
  final String title;
  const Category(this.title);

  @override
  List<Category> get children {
    return [];
  }
}

class SidebarBrowser extends StatefulWidget {
  const SidebarBrowser({
    Key? key,
  }) : super(key: key);

  @override
  State<SidebarBrowser> createState() => _SidebarBrowserState();
}

class _SidebarBrowserState extends State<SidebarBrowser> {
  Category? selectedCategory;

  @override
  Widget build(BuildContext context) {
    return Row(children: [
      SizedBox(
        width: 150,
        child: Container(
          decoration: const BoxDecoration(
              border: Border(
                  right: BorderSide(color: Color.fromRGBO(20, 20, 20, 1.0))),
              color: Color.fromRGBO(80, 80, 80, 1.0)),
          child: SidebarCategories(
              selectedCategory: selectedCategory,
              onSelectCategory: onSelectCategory),
        ),
      ),
      Expanded(
        child: Container(
          padding: const EdgeInsets.all(4.0),
          decoration: const BoxDecoration(
              border: Border(
                  right: BorderSide(color: Color.fromRGBO(20, 20, 20, 1.0)))),
          child: ListView(children: const [
            Text("Samples"),
            Text("Effects"),
            Text("Instruments"),
            Text("Plugins"),
          ]),
        ),
      )
    ]);
  }

  void onSelectCategory(Category category) {
    setState(() {
      selectedCategory = category;
    });
  }
}

class SidebarCategories extends StatelessWidget {
  final List<Category> categories = const [
    Category("Samples"),
    Category("Effects"),
    Category("Instruments"),
    Category("Plug-ins"),
  ];
  final Category? selectedCategory;
  final void Function(Category) onSelectCategory;

  const SidebarCategories(
      {Key? key,
      required this.selectedCategory,
      required this.onSelectCategory})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SidebarButtonsListView(
        values: categories,
        selectedValue: selectedCategory,
        onSelect: onSelectCategory);
  }
}
