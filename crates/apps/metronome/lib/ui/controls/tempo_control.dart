import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:graphx/graphx.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/ui/controls/tempo_control/keyboard_overlay.dart';
import 'package:metronome/ui/utils/debounce.dart';
import 'package:mobx/mobx.dart';

class TempoControl extends StatefulWidget {
  const TempoControl({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  final MetronomeStateController stateController;

  @override
  State<TempoControl> createState() => _TempoControlState();
}

class _TempoControlState extends State<TempoControl> {
  late TextEditingController _textEditingController;
  final Debounce _onChangeDebounce = Debounce(1000);
  final FocusNode _inputFocusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _textEditingController = TextEditingController(
      text: widget.stateController.model.tempo.toStringAsFixed(0),
    );

    autorun((_) {
      _textEditingController.value = _textEditingController.value.copyWith(
        text: widget.stateController.model.tempo.toStringAsFixed(0),
      );
    });

    _inputFocusNode.addListener(() {
      if (_inputFocusNode.hasFocus) {
        KeyboardOverlay.showOverlay(context, _inputFocusNode.parent);
      } else {
        KeyboardOverlay.removeOverlay();
      }
    });
  }

  @override
  void dispose() {
    super.dispose();
    _onChangeDebounce.cancel();
    _inputFocusNode.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final model = widget.stateController.model;
    return Observer(
      builder: (_) => Column(
        children: [
          const Text("Tempo"),
          Row(
            children: [
              CupertinoButton(
                child: const Text("-10"),
                onPressed: () {
                  widget.stateController
                      .setTempo(widget.stateController.model.tempo - 10);

                  final analytics = AppContext.of(context).analytics;
                  analytics.logEvent(
                    name: "TempoControl__quickTempoChange",
                  );
                },
              ),
              Expanded(
                child: CupertinoTextField.borderless(
                  autocorrect: false,
                  keyboardType: TextInputType.number,
                  focusNode: _inputFocusNode,
                  onSubmitted: (value) {
                    _inputFocusNode.parent?.requestFocus();
                  },
                  textInputAction: TextInputAction.done,
                  style: const TextStyle(fontSize: 80.0),
                  controller: _textEditingController,
                  textAlign: TextAlign.center,
                  cursorWidth: 1.0,
                  onChanged: onTempoTextChanged,
                  onEditingComplete: onTempoTextEditingComplete,
                ),
              ),
              CupertinoButton(
                child: const Text("+10"),
                onPressed: () {
                  widget.stateController
                      .setTempo(widget.stateController.model.tempo + 10);

                  final analytics = AppContext.of(context).analytics;
                  analytics.logEvent(
                    name: "TempoControl__quickTempoChange",
                  );
                },
              ),
            ],
          ),
          SizedBox(
            width: double.infinity,
            child: CupertinoSlider(
              value: Math.min(Math.max(1, model.tempo), 350),
              onChanged: (value) {
                widget.stateController.setTempo(value);
              }, // onTempoChanged,
              onChangeEnd: (value) {
                final analytics = AppContext.of(context).analytics;
                analytics.logEvent(
                  name: "TempoControl__sliderTempoChanged",
                );
              },
              min: 1,
              max: 350,
            ),
          )
        ],
      ),
    );
  }

  void onTempoTextChanged(String value) {
    _onChangeDebounce.run(() {
      final double tempo = Math.max(Math.min(double.parse(value), 350), 1);
      widget.stateController.setTempo(tempo);

      final analytics = AppContext.of(context).analytics;
      analytics.logEvent(name: "TempoControl__onTempoTextChanged");
    });
  }

  void onTempoTextEditingComplete() {
    _onChangeDebounce.flush();

    final analytics = AppContext.of(context).analytics;
    analytics.logEvent(name: "TempoControl__onTempoTextEditingComplete");
  }
}
