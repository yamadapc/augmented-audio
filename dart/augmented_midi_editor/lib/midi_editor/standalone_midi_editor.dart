import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'midi_editor.dart';
import 'midi_editor_view_model.dart';

class StandaloneMIDIEditorView extends StatelessWidget {
  final MIDIEditorViewModel midiEditorViewModel = MIDIEditorViewModel();

  StandaloneMIDIEditorView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      SizedBox(
          height: 300,
          child: SingleChildScrollView(
              controller: ScrollController(),
              child: StandaloneMIDISettingsView(model: midiEditorViewModel))),
      Expanded(child: MIDIEditorView(model: midiEditorViewModel))
    ]);
  }
}

class StandaloneMIDISettingsView extends StatelessWidget {
  final MIDIEditorViewModel model;

  const StandaloneMIDISettingsView({Key? key, required this.model})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text("JSON Representation:"),
            Container(
              width: double.infinity,
              margin: const EdgeInsets.only(top: 16.0),
              decoration: BoxDecoration(
                  color: Colors.black,
                  border: Border.all(color: Colors.white)),
              padding: const EdgeInsets.all(8.0),
              child: Observer(builder: (context) {
                var textStyle = DefaultTextStyle.of(context)
                    .style
                    .merge(const TextStyle(
                      fontFamily: "Monaco",
                    ));
                var encoder = const JsonEncoder.withIndent("  ");
                var jsonString = encoder.convert(model.toJson());
                return Text(jsonString, style: textStyle);
              }),
            ),
            const SizedBox(height: 10),
          ]),
    );
  }
}
