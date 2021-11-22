import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/dev/storybook.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

class FilePickerView extends StatelessWidget {
  final void Function(FilePickerResult? result) onFilePicked;

  const FilePickerView({Key? key, required this.onFilePicked})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return TextButton(onPressed: onPressed, child: const Text("Open file..."));
  }

  Future<void> onPressed() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles();
    onFilePicked(result);
  }
}

class FilePickerStory extends StatelessWidget {
  final Observable<FilePickerResult?> result = Observable(null);

  FilePickerStory({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
        builder: (_) => SizedBox(
              width: 300,
              child: Column(children: [
                FilePickerView(
                    onFilePicked: (FilePickerResult? filePickerResult) {
                  runInAction(() {
                    result.value = filePickerResult;
                  });
                }),
                Text("${result.value}")
              ]),
            ));
  }
}

Story filePickerStory() =>
    story("File picker").view(() => FilePickerStory()).build();
