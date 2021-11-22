import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/audio_io_state.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'track_controls/knob_field.dart';
import 'track_controls/volume_meter.dart';

class TrackControls extends StatelessWidget {
  final Track track;

  const TrackControls({
    Key? key,
    required this.track,
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
                                children: [
                                  KnobField(label: "Pan", model: track.pan),
                                  KnobField(label: "A", model: track.sends[0]),
                                ]),
                          ),
                        ),
                        SizedBox(width: 30, child: VolumeMeter()),
                      ],
                    ),
                  ),
                  Observer(
                    builder: (_) => AudioIOInputDropdownView(
                        value: AudioIOStateProvider.stateOf(context)
                            .availableInputs
                            .firstWhere(
                                (input) => input.id == track.audioInputId),
                        onChanged: onAudioInputChanged),
                  )
                ]),
          )),
    );
  }

  void onAudioInputChanged(AudioInput? input) {
    if (input != null) {
      track.setAudioInputId(input.id);
    } else {
      track.setAudioInputId("none");
    }
  }
}

typedef AudioIOInputDropdownViewOnChanged = void Function(AudioInput?);

class AudioIOInputDropdownView extends StatelessWidget {
  final AudioInput? value;
  final AudioIOInputDropdownViewOnChanged onChanged;

  const AudioIOInputDropdownView({
    Key? key,
    required this.value,
    required this.onChanged,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (buildContext) {
        var audioIOState = AudioIOStateProvider.stateOf(buildContext);
        var dropdownItems = audioIOState.availableInputs
            .map(
              (input) =>
                  DropdownMenuItem(child: Text(input.title), value: input),
            )
            .toList();

        return DropdownButton<AudioInput>(
            dropdownColor: const Color.fromRGBO(30, 30, 30, 1.0),
            style: DefaultTextStyle.of(buildContext)
                .style
                .merge(TextStyle(color: Colors.white.withOpacity(0.8))),
            isExpanded: true,
            value: value,
            items: dropdownItems,
            onChanged: onChanged);
      },
    );
  }
}
