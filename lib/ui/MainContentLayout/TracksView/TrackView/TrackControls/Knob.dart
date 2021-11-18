import 'package:flutter/material.dart';

class Knob extends StatelessWidget {
  const Knob({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        height: 50,
        width: 50,
        decoration: const BoxDecoration(
          color: Colors.black,
          shape: BoxShape.circle,
        ),
        child: null);
  }
}
