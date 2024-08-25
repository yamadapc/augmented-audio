import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';

class PlatformSegmentedControl<T extends Object> extends StatelessWidget {
  final T value;
  final List<T> options;
  final String Function(T) optionLabelBuilder;
  final void Function(T) onValueChanged;

  const PlatformSegmentedControl({
    super.key,
    required this.value,
    required this.options,
    required this.optionLabelBuilder,
    required this.onValueChanged,
  });

  @override
  Widget build(BuildContext context) {
    return PlatformWidget(
      cupertino: (_, __) => CupertinoSegmentedControl<T>(
        groupValue: value,
        onValueChanged: (value) {
          onValueChanged(value);
        },
        children: Map.fromEntries(
          options.map((e) {
            return MapEntry<T, Widget>(
              e,
              cupertinoSegmentedControlText(optionLabelBuilder(e)),
            );
          }),
        ),
      ),
      material: (_, __) => SegmentedButton(
        showSelectedIcon: false,
        style: ButtonStyle(
          backgroundColor: WidgetStateProperty.all<Color>(
            CupertinoColors.systemBlue.color,
          ),
        ),
        segments: options.map((option) {
          return ButtonSegment(
            label: Text(optionLabelBuilder(option)),
            value: option,
          );
        }).toList(),
        onSelectionChanged: (Set<T> value) {
          onValueChanged(value.first);
        },
        selected: {value},
      ),
    );
  }
}

Widget cupertinoSegmentedControlText(String s) {
  return Padding(
    padding: const EdgeInsets.only(left: 3.0, right: 3.0),
    child: Text(s, textScaleFactor: Platform.isMacOS ? 0.85 : 1.0),
  );
}
