import 'package:flutter/material.dart';

import 'track_controls/knob_field.dart';
import 'track_controls/volume_meter.dart';

class TrackControls extends StatelessWidget {
  const TrackControls({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var defaultTextStyle = DefaultTextStyle.of(context).style;
    var textStyle =
        defaultTextStyle.merge(TextStyle(color: Colors.white.withOpacity(0.8)));
    return SizedBox(
      width: double.infinity,
      child: Container(
          padding: const EdgeInsets.all(8.0),
          decoration: const BoxDecoration(
            color: Color.fromRGBO(60, 60, 60, 1.0),
            border:
                Border(top: BorderSide(color: Color.fromRGBO(90, 90, 90, 1.0))),
          ),
          child: DefaultTextStyle.merge(
            style: textStyle,
            child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  IntrinsicHeight(
                    child: Row(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Expanded(
                          child: Container(
                            padding: const EdgeInsets.only(right: 8.0),
                            child: Column(
                                mainAxisAlignment:
                                    MainAxisAlignment.spaceEvenly,
                                crossAxisAlignment: CrossAxisAlignment.center,
                                children: const [
                                  KnobField(label: "Pan"),
                                  KnobField(label: "A"),
                                ]),
                          ),
                        ),
                        const SizedBox(width: 30, child: VolumeMeter()),
                      ],
                    ),
                  ),
                  DropdownButton(
                      dropdownColor: const Color.fromRGBO(30, 30, 30, 1.0),
                      style: textStyle,
                      isExpanded: true,
                      value: "Input 1",
                      items: const [
                        DropdownMenuItem(
                            child: Text("Input 1"), value: "Input 1"),
                        DropdownMenuItem(
                            child: Text("Input 2"), value: "Input 2"),
                        DropdownMenuItem(
                            child: Text("Input 3"), value: "Input 3"),
                      ],
                      onChanged: onChanged)
                ]),
          )),
    );
  }

  void onChanged(Object? value) {}
}
