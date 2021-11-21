import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';

class StatusBar extends StatelessWidget {
  const StatusBar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
      child: Container(
        height: 20,
        width: double.infinity,
        padding: const EdgeInsets.only(left: 8, right: 8),
        decoration: const BoxDecoration(color: Colors.black),
        child: const Text("Loaded"),
      ),
    );
  }
}
