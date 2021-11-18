import 'package:flutter/material.dart';

class DawTextStyle extends StatelessWidget {
  const DawTextStyle({Key? key, required this.child}) : super(key: key);

  final Widget child;

  @override
  Widget build(BuildContext context) {
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return DefaultTextStyle(style: textStyle, child: child);
  }
}
