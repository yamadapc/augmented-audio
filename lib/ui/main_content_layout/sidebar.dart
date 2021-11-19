import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
      child: SizedBox(
        width: 400,
        child: Container(
            decoration:
                const BoxDecoration(color: Color.fromRGBO(50, 50, 50, 1.0)),
            child: const SidebarBrowser()),
      ),
    );
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
      SizedBox(
        width: 250,
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
    return ListView(
        children: categories
            .map((category) => SidebarCategoryView(
                category: category,
                isSelected: category == selectedCategory,
                onPressed: onSelectCategory))
            .toList());
  }
}

class Category {
  final String title;

  const Category(this.title);
}

class SidebarCategoryView extends StatelessWidget {
  final Category category;
  final bool isSelected;
  final void Function(Category category) onPressed;

  const SidebarCategoryView(
      {Key? key,
      required this.category,
      required this.isSelected,
      required this.onPressed})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        decoration: BoxDecoration(
            color: isSelected
                ? const Color.fromRGBO(20, 20, 20, 1.0)
                : Colors.transparent),
        padding: const EdgeInsets.only(top: 4, bottom: 4, left: 8, right: 8),
        child: TextButton(
            style: ButtonStyle(
                foregroundColor: MaterialStateProperty.all(Colors.white),
                backgroundColor: MaterialStateProperty.all(Colors.transparent),
                alignment: Alignment.centerLeft,
                textStyle: MaterialStateProperty.all(
                    const TextStyle(color: Colors.white))),
            onPressed: onPressedInner,
            child: Text(category.title)));
  }

  void onPressedInner() {
    onPressed(category);
  }
}
