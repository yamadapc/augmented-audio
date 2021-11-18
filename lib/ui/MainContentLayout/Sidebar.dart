import 'package:flutter/material.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      child: Container(
          decoration:
              const BoxDecoration(color: Color.fromRGBO(50, 50, 50, 1.0)),
          child: null),
    );
  }
}
