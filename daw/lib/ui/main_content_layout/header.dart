import 'package:flutter/material.dart';

class Header extends StatelessWidget {
  const Header({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = TextStyle(color: Colors.white.withOpacity(0.8));
    return DefaultTextStyle.merge(
      style: textStyle,
      child: Container(
          width: double.infinity,
          padding: const EdgeInsets.all(4.0),
          decoration: BoxDecoration(boxShadow: [
            BoxShadow(
              color: Colors.black.withOpacity(0.4),
              spreadRadius: 1.0,
              blurRadius: 5.0,
            )
          ], border: Border.all(color: Colors.black)),
          child: const TransportControls()),
    );
  }
}

class TransportControls extends StatelessWidget {
  const TransportControls({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: const [
          Icon(Icons.stop, color: Colors.white),
          Icon(Icons.play_arrow, color: Colors.white),
          Icon(Icons.fiber_manual_record, color: Colors.white),
        ]);
  }
}
