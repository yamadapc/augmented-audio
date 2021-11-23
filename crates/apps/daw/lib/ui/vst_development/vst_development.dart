import 'package:file_picker/file_picker.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter_daw_mock_ui/state/wire.dart';
import 'package:flutter_daw_mock_ui/ui/common/file_picker.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';

class VSTDevelopmentView extends StatelessWidget {
  const VSTDevelopmentView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
        child: Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        children: [
          Row(children: [
            Container(
                alignment: Alignment.centerRight,
                width: 300,
                child: const Text("Input file")),
            FilePickerView(onFilePicked: onInputFilePicked),
          ]),
          Row(children: [
            Container(
                alignment: Alignment.centerRight,
                width: 300,
                child: const Text("VST path")),
            FilePickerView(onFilePicked: onPluginFilePicked),
          ]),
          const Heading(text: "Logs"),
        ],
      ),
    ));
  }

  void onInputFilePicked(FilePickerResult? result) {
    var path = result?.files[0].path;
    if (path != null) {
      dawUi?.setInputFilePath(path: path);
    }
  }

  void onPluginFilePicked(FilePickerResult? result) {
    var path = result?.files[0].path;
    if (path != null) {
      dawUi?.setVstFilePath(path: path);
    }
  }
}
