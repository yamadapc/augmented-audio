import 'package:flutter/material.dart';

class DawTextStyle extends StatelessWidget {
  const DawTextStyle({Key? key, required this.child}) : super(key: key);

  final Widget child;

  @override
  Widget build(BuildContext context) {
    var textStyle = of(context);
    return DefaultTextStyle(style: textStyle, child: child);
  }

  static TextStyle of(BuildContext context) {
    return DefaultTextStyle.of(context)
        .style
        .merge(TextStyle(color: Colors.white.withOpacity(0.8)));
  }
}

class Heading extends StatelessWidget {
  final String text;

  const Heading({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = DawTextStyle.of(context);

    return Container(
      alignment: Alignment.centerLeft,
      child: Text(text,
          style: textStyle.merge(const TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold,
          ))),
    );
  }
}
