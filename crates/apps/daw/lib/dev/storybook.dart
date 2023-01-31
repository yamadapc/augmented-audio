// ignore_for_file: library_private_types_in_public_api

import 'package:flutter/widgets.dart';
import 'package:flutter_daw_mock_ui/ui/common/generic_sidebar.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

part 'storybook.g.dart';

class StorybookState = _StorybookState with _$StorybookState;

abstract class _StorybookState with Store {
  @observable
  Storybook storybook = rootStory;

  @observable
  String? selectedStoryId;

  @computed
  Story? get selectedStory {
    var id = selectedStoryId;
    if (id == null) {
      return null;
    }
    return findStory(id, storybook.stories);
  }
}

Story? findStory(String id, List<Story> tree) {
  for (var value in tree) {
    if (value.title == id) {
      return value;
    }

    var candidate = findStory(id, value.children);
    if (candidate != null) {
      return candidate;
    }
  }
  return null;
}

class Storybook = _Storybook with _$Storybook;

abstract class _Storybook with Store {
  @observable
  List<Story> stories = [];

  @action
  void addStory(Story story) {
    if (stories.contains(story)) {
      return;
    }
    stories.add(story);
  }
}

class StorybookView extends StatelessWidget {
  final StorybookState storybookState;

  const StorybookView({Key? key, required this.storybookState})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
      child: Row(children: [
        SizedBox(
            width: 150,
            child: Container(
                decoration: BoxDecoration(
                    color: const Color.fromRGBO(30, 30, 30, 1),
                    border: Border.all(
                        color: const Color.fromRGBO(20, 20, 20, 1.0))),
                child: StorybookSidebarView(storybookState: storybookState))),
        Expanded(
          child: StorybookContentView(storybookState: storybookState),
        )
      ]),
    );
  }
}

class StorybookSidebarView extends StatelessWidget {
  final StorybookState storybookState;

  const StorybookSidebarView({Key? key, required this.storybookState})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => SidebarButtonsListView(
          values: storybookState.storybook.stories,
          selectedValue: storybookState.selectedStory,
          onSelect: onSelect),
    );
  }

  void onSelect(Story value) {
    storybookState.selectedStoryId = value.title;
  }
}

class StorybookContentView extends StatelessWidget {
  final StorybookState storybookState;

  const StorybookContentView({Key? key, required this.storybookState})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(builder: (_) {
      var currentView = storybookState.selectedStory?.view;
      if (currentView != null) {
        return currentView();
      }
      return const Center(child: Text("Select a story"));
    });
  }
}

class Story implements SidebarItem<Story> {
  @override
  final String title;
  final WidgetBuilder? view;
  @override
  final List<Story> children;

  Story(this.title, this.view, this.children);
}

typedef WidgetBuilder = Widget Function();
typedef ChildBuilder = void Function(StoryBuilder builder);

class StoryBuilder {
  String _title = "untitled";
  WidgetBuilder? _view;
  final List<Story> _children = [];

  StoryBuilder title(String title) {
    _title = title;
    return this;
  }

  StoryBuilder view(WidgetBuilder widget) {
    _view = widget;
    return this;
  }

  StoryBuilder child(String title, ChildBuilder builder) {
    StoryBuilder storyBuilder = StoryBuilder().title(title);
    builder(storyBuilder);
    _children.add(storyBuilder.build());
    return this;
  }

  Story build() {
    var story = Story(_title, _view, _children);
    return story;
  }
}

var rootStory = Storybook();

StoryBuilder story([String? title]) {
  StoryBuilder storyBuilder = StoryBuilder();

  if (title != null) {
    storyBuilder.title(title);
  }

  return storyBuilder;
}
