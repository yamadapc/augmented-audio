import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';

class BottomPanel extends StatelessWidget {
  const BottomPanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
      child: Container(
        decoration: BoxDecoration(boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.4),
            spreadRadius: 1.0,
            blurRadius: 5.0,
          )
        ], border: Border.all(color: const Color.fromRGBO(65, 65, 65, 1.0))),
        height: 200,
        child: Row(children: const [Text("Plugin 1")]),
      ),
    );
  }
}
