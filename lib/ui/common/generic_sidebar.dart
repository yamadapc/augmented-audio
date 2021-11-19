import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/dev/storybook.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

abstract class SidebarItem<T extends SidebarItem<T>> {
  String get title;
  List<T> get children {
    return [];
  }
}

class SidebarButtonsListView<T extends SidebarItem<T>> extends StatelessWidget {
  final List<T> values;
  final T? selectedValue;
  final void Function(T value) onSelect;

  const SidebarButtonsListView({
    Key? key,
    required this.values,
    required this.selectedValue,
    required this.onSelect,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListView(
        children: values.map((value) {
      return SidebarItemView(
          value: value, selectedValue: selectedValue, onSelect: onSelect);
    }).toList());
  }
}

class SidebarItemView<T extends SidebarItem<T>> extends StatelessWidget {
  final T value;
  final T? selectedValue;
  final void Function(T value) onSelect;

  const SidebarItemView(
      {Key? key,
      required this.value,
      required this.selectedValue,
      required this.onSelect})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (value.children.isNotEmpty) {
      var children = value.children
          .map((child) => SidebarItemView(
              value: child, selectedValue: selectedValue, onSelect: onSelect))
          .toList();
      return Column(
        children: [
          SidebarButtonView(
              title: value.title,
              value: value,
              isSelected: value == selectedValue,
              onPressed: onSelect),
          Container(
            padding: const EdgeInsets.only(left: 16.0),
            child: Column(
              children: children,
            ),
          )
        ],
      );
    }

    return SidebarButtonView(
        title: value.title,
        value: value,
        isSelected: selectedValue == value,
        onPressed: onSelect);
  }
}

class SidebarButtonView<T> extends StatelessWidget {
  final String title;
  final T value;
  final bool isSelected;
  final void Function(T value) onPressed;

  const SidebarButtonView(
      {Key? key,
      required this.title,
      required this.value,
      required this.isSelected,
      required this.onPressed})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        width: double.infinity,
        decoration: BoxDecoration(
            color: isSelected
                ? const Color.fromRGBO(20, 20, 20, 1.0)
                : Colors.transparent),
        padding: const EdgeInsets.only(top: 0, bottom: 4, left: 4, right: 4),
        child: TextButton(
            style: ButtonStyle(
                foregroundColor: MaterialStateProperty.all(Colors.white),
                backgroundColor: MaterialStateProperty.all(Colors.transparent),
                alignment: Alignment.centerLeft,
                textStyle: MaterialStateProperty.all(
                    const TextStyle(color: Colors.white))),
            onPressed: onPressedInner,
            child: Text(title)));
  }

  void onPressedInner() {
    onPressed(value);
  }
}

class StringSidebarItem implements SidebarItem<StringSidebarItem> {
  @override
  final String title;
  @override
  final List<StringSidebarItem> children;

  StringSidebarItem(this.title, this.children);
}

class SidebarStory extends StatelessWidget {
  const SidebarStory({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var values = ["Hello", "Here how are you", "Something"]
        .map((v) => StringSidebarItem(v, []))
        .toList();
    Observable<StringSidebarItem?> selectedValue = Observable(null);
    return Observer(
      builder: (_) {
        return SidebarButtonsListView<StringSidebarItem>(
            values: values,
            selectedValue: selectedValue.value,
            onSelect: (value) {
              runInAction(() {
                selectedValue.value = value;
              });
            });
      },
    );
  }
}

class NestedSidebarStory extends StatelessWidget {
  const NestedSidebarStory({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var values = ["Hello", "Here how are you", "Something"]
        .map((v) => StringSidebarItem(
            v,
            ["Other nested", "items"]
                .map((e) => StringSidebarItem(e, []))
                .toList()))
        .toList();
    Observable<StringSidebarItem?> selectedValue = Observable(null);
    return Observer(
      builder: (_) {
        return SidebarButtonsListView<StringSidebarItem>(
            values: values,
            selectedValue: selectedValue.value,
            onSelect: (value) {
              runInAction(() {
                selectedValue.value = value;
              });
            });
      },
    );
  }
}

Story sidebarStory() => story("Generic Sidebar").child("Simple", (builder) {
      builder.view(() => const SidebarStory());
    }).child("Nested", (builder) {
      builder.view(() => const NestedSidebarStory());
    }).build();
