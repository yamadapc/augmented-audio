import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/audio_io_state.dart';
import 'package:flutter_daw_mock_ui/ui/common/generic_sidebar.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

part 'settings_view.g.dart';

class SettingsState = _SettingsState with _$SettingsState;

abstract class _SettingsState with Store {
  @observable
  String selectedTab = "Audio Settings";

  @action
  void setSelectedTab(String tab) {
    selectedTab = tab;
  }
}

var settingsState = SettingsState();

class SettingsView extends StatelessWidget {
  const SettingsView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var tabs = [StringSidebarItem("Audio Settings", [])];

    var content = const AudioSettingsView();

    return Row(children: [
      Container(
        decoration: const BoxDecoration(color: Color.fromRGBO(90, 90, 90, 1)),
        width: 200,
        child: SidebarButtonsListView(
            values: tabs,
            selectedValue: tabs[0],
            onSelect: (StringSidebarItem value) {
              settingsState.setSelectedTab(value.title);
            }),
      ),
      Expanded(child: DawTextStyle(child: content))
    ]);
  }
}

class AudioSettingsView extends StatelessWidget {
  const AudioSettingsView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = DawTextStyle.of(context);
    var audioIOState = AudioIOStateProvider.stateOf(context);

    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text("Audio settings",
              style: textStyle.merge(const TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
              ))),
          const SizedBox(height: 10),
          Observer(
            builder: (_) => FormFieldView<AudioDevice>(
              value: audioIOState.currentInputDevice,
              label: "Input device",
              hint: "Select audio input device...",
              options: audioIOState.inputDevices
                  .map((inputDevice) =>
                      FormFieldOption(inputDevice.title, inputDevice))
                  .toList(),
              onChanged: (inputDevice) {
                audioIOState.setInputDevice(inputDevice);
              },
            ),
          ),
          Observer(
            builder: (_) => FormFieldView<AudioDevice>(
              value: audioIOState.currentOutputDevice,
              label: "Output device",
              hint: "Select audio output device...",
              options: audioIOState.outputDevices
                  .map((outputDevice) =>
                      FormFieldOption(outputDevice.title, outputDevice))
                  .toList(),
              onChanged: (outputDevice) {
                audioIOState.setOutputDevice(outputDevice);
              },
            ),
          ),
        ],
      ),
    );
  }
}

class FormFieldOption<T> {
  final String label;
  final T value;

  FormFieldOption(this.label, this.value);
}

class FormFieldView<T> extends StatelessWidget {
  final String label;
  final String hint;
  final List<FormFieldOption<T>> options;
  final void Function(T?) onChanged;
  final T? value;

  const FormFieldView(
      {Key? key,
      required this.value,
      required this.label,
      required this.hint,
      required this.options,
      required this.onChanged})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textStyle = DawTextStyle.of(context);

    return Row(
      children: [
        ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 120),
          child: Container(
            padding: const EdgeInsets.only(left: 16),
            alignment: Alignment.centerRight,
            child: Text(label,
                style: textStyle
                    .merge(const TextStyle(fontWeight: FontWeight.bold))),
          ),
        ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(left: 16, right: 16),
            child: DropdownButton<T>(
                isExpanded: true,
                value: value,
                dropdownColor: const Color.fromRGBO(30, 30, 30, 1.0),
                style: textStyle,
                hint: Text(hint,
                    style: textStyle
                        .merge(const TextStyle(fontStyle: FontStyle.italic))),
                items: options
                    .map(
                      (option) => DropdownMenuItem(
                          child: Text(option.label, style: textStyle),
                          value: option.value),
                    )
                    .toList(),
                onChanged: onChanged),
          ),
        ),
      ],
    );
  }
}
