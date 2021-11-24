import 'package:flutter/material.dart';

class PianoKeyView extends StatelessWidget {
  final bool isSharp;

  const PianoKeyView({Key? key, required this.isSharp}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.only(left: 8),
        width: 50,
        height: 20,
        decoration: BoxDecoration(
          color: isSharp ? Colors.black : Colors.white,
        ),
        child: null);
  }
}
