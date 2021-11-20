import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/dev/storybook.dart';

class FilePickerView extends StatelessWidget {
  const FilePickerView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return TextButton(onPressed: onPressed, child: const Text("Open file..."));
  }

  Future<void> onPressed() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles();
    print(result);
  }
}

class FilePickerStory extends StatelessWidget {
  const FilePickerStory({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const FilePickerView();
  }
}

Story filePickerStory() =>
    story("File picker").view(() => const FilePickerStory()).build();
