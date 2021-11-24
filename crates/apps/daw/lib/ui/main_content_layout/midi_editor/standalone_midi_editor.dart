import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:flutter_daw_mock_ui/state/wire/wire.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_editor.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_editor_view_model.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

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
    return DawTextStyle(
        child: Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Heading(text: "Settings"),
          const SizedBox(height: 10),
          Row(
            children: [
              Expanded(
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
              ),
              const Expanded(child: SynthesizerView())
            ],
          )
        ],
      ),
    ));
  }
}

class SynthesizerView extends StatefulWidget {
  const SynthesizerView({Key? key}) : super(key: key);

  @override
  State<SynthesizerView> createState() => _SynthesizerViewState();
}

class _SynthesizerViewState extends State<SynthesizerView> {
  SynthesizerApi? synth;

  @override
  void initState() {
    var api = initialize();
    api?.audioGraphSetup();
    synth = SynthesizerApi(api);
  }

  @override
  Widget build(BuildContext context) {
    return Container(child: null);
  }
}

class SynthesizerApi {
  final DawUi? api;

  SynthesizerApi(this.api);
}
