import 'package:flutter/cupertino.dart';
import 'package:flutter_daw_mock_ui/dev/storybook.dart';

import 'common/file_picker.dart';
import 'common/generic_sidebar.dart';

StorybookState storybookState = StorybookState();

class DawStorybook extends StatelessWidget {
  const DawStorybook({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    setupStories();
    return StorybookView(storybookState: storybookState);
  }

  void setupStories() {
    rootStory.stories.clear();
    rootStory.addStory(sidebarStory());
    rootStory.addStory(filePickerStory());
    storybookState.storybook = rootStory;
  }
}
