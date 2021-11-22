import 'package:flutter/material.dart';

class ClipSlot extends StatelessWidget {
  const ClipSlot({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 35,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
              color: Color.fromRGBO(50, 50, 50, 1),
              border: Border(bottom: BorderSide(color: Colors.black))),
          child: null),
    );
  }
}

class ClipView extends StatelessWidget {
  final String title;
  const ClipView({Key? key, required this.title}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: double.infinity,
      height: 35,
      child: Container(
          decoration: const BoxDecoration(
              color: Color.fromRGBO(50, 50, 50, 1),
              border: Border(bottom: BorderSide(color: Colors.black))),
          child: Container(
              margin: const EdgeInsets.all(1.0),
              padding: const EdgeInsets.only(left: 6.0, right: 6.0),
              decoration: const BoxDecoration(
                  color: Color.fromRGBO(89, 199, 228, 1),
                  border: Border(bottom: BorderSide(color: Colors.black))),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Expanded(child: Text(title)),
                  SizedBox(
                    height: 20,
                    width: 20,
                    child: IconButton(
                        padding: const EdgeInsets.all(0),
                        onPressed: () {},
                        icon: const Icon(
                          Icons.stop,
                          size: 20,
                        )),
                  ),
                ],
              ))),
    );
  }
}
