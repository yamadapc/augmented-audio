// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'storybook.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$StorybookState on _StorybookState, Store {
  Computed<Story?>? _$selectedStoryComputed;

  @override
  Story? get selectedStory =>
      (_$selectedStoryComputed ??= Computed<Story?>(() => super.selectedStory,
              name: '_StorybookState.selectedStory'))
          .value;

  final _$storybookAtom = Atom(name: '_StorybookState.storybook');

  @override
  Storybook get storybook {
    _$storybookAtom.reportRead();
    return super.storybook;
  }

  @override
  set storybook(Storybook value) {
    _$storybookAtom.reportWrite(value, super.storybook, () {
      super.storybook = value;
    });
  }

  final _$selectedStoryIdAtom = Atom(name: '_StorybookState.selectedStoryId');

  @override
  String? get selectedStoryId {
    _$selectedStoryIdAtom.reportRead();
    return super.selectedStoryId;
  }

  @override
  set selectedStoryId(String? value) {
    _$selectedStoryIdAtom.reportWrite(value, super.selectedStoryId, () {
      super.selectedStoryId = value;
    });
  }

  @override
  String toString() {
    return '''
storybook: ${storybook},
selectedStoryId: ${selectedStoryId},
selectedStory: ${selectedStory}
    ''';
  }
}

mixin _$Storybook on _Storybook, Store {
  final _$storiesAtom = Atom(name: '_Storybook.stories');

  @override
  List<Story> get stories {
    _$storiesAtom.reportRead();
    return super.stories;
  }

  @override
  set stories(List<Story> value) {
    _$storiesAtom.reportWrite(value, super.stories, () {
      super.stories = value;
    });
  }

  final _$_StorybookActionController = ActionController(name: '_Storybook');

  @override
  void addStory(Story story) {
    final _$actionInfo =
        _$_StorybookActionController.startAction(name: '_Storybook.addStory');
    try {
      return super.addStory(story);
    } finally {
      _$_StorybookActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
stories: ${stories}
    ''';
  }
}
