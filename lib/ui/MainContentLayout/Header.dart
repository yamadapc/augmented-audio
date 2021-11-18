import 'package:flutter/material.dart';

class Header extends StatelessWidget {
  const Header({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return DefaultTextStyle.merge(
      style: textStyle,
      child: Container(
          height: 30,
          width: double.infinity,
          padding: const EdgeInsets.all(4.0),
          decoration: BoxDecoration(boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.4),
              spreadRadius: 1.0,
              blurRadius: 5.0,
            )
          ], border: Border.all(color: Colors.black)),
          child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: const [Text("DAW")])),
    );
  }
}
