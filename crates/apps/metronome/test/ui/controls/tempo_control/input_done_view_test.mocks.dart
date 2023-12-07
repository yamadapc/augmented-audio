// Mocks generated by Mockito 5.4.2 from annotations
// in metronome/test/ui/controls/tempo_control/input_done_view_test.dart.
// Do not manually edit this file.

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'dart:ui' as _i2;

import 'package:flutter/foundation.dart' as _i4;
import 'package:flutter/src/widgets/focus_manager.dart' as _i3;
import 'package:flutter/src/widgets/focus_traversal.dart' as _i6;
import 'package:flutter/src/widgets/framework.dart' as _i5;
import 'package:mockito/mockito.dart' as _i1;

// ignore_for_file: type=lint
// ignore_for_file: avoid_redundant_argument_values
// ignore_for_file: avoid_setters_without_getters
// ignore_for_file: comment_references
// ignore_for_file: implementation_imports
// ignore_for_file: invalid_use_of_visible_for_testing_member
// ignore_for_file: prefer_const_constructors
// ignore_for_file: unnecessary_parenthesis
// ignore_for_file: camel_case_types
// ignore_for_file: subtype_of_sealed_class

class _FakeSize_0 extends _i1.SmartFake implements _i2.Size {
  _FakeSize_0(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeOffset_1 extends _i1.SmartFake implements _i2.Offset {
  _FakeOffset_1(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeRect_2 extends _i1.SmartFake implements _i2.Rect {
  _FakeRect_2(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeFocusAttachment_3 extends _i1.SmartFake
    implements _i3.FocusAttachment {
  _FakeFocusAttachment_3(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );
}

class _FakeDiagnosticsNode_4 extends _i1.SmartFake
    implements _i4.DiagnosticsNode {
  _FakeDiagnosticsNode_4(
    Object parent,
    Invocation parentInvocation,
  ) : super(
          parent,
          parentInvocation,
        );

  @override
  String toString({
    _i4.TextTreeConfiguration? parentConfiguration,
    _i4.DiagnosticLevel? minLevel = _i4.DiagnosticLevel.info,
  }) =>
      super.toString();
}

/// A class which mocks [FocusNode].
///
/// See the documentation for Mockito's code generation for more information.
class MockFocusNode extends _i1.Mock implements _i3.FocusNode {
  @override
  set onKey(_i3.FocusOnKeyCallback? _onKey) => super.noSuchMethod(
        Invocation.setter(
          #onKey,
          _onKey,
        ),
        returnValueForMissingStub: null,
      );

  @override
  set onKeyEvent(_i3.FocusOnKeyEventCallback? _onKeyEvent) =>
      super.noSuchMethod(
        Invocation.setter(
          #onKeyEvent,
          _onKeyEvent,
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool get skipTraversal => (super.noSuchMethod(
        Invocation.getter(#skipTraversal),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  set skipTraversal(bool? value) => super.noSuchMethod(
        Invocation.setter(
          #skipTraversal,
          value,
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool get canRequestFocus => (super.noSuchMethod(
        Invocation.getter(#canRequestFocus),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  set canRequestFocus(bool? value) => super.noSuchMethod(
        Invocation.setter(
          #canRequestFocus,
          value,
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool get descendantsAreFocusable => (super.noSuchMethod(
        Invocation.getter(#descendantsAreFocusable),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  set descendantsAreFocusable(bool? value) => super.noSuchMethod(
        Invocation.setter(
          #descendantsAreFocusable,
          value,
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool get descendantsAreTraversable => (super.noSuchMethod(
        Invocation.getter(#descendantsAreTraversable),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  set descendantsAreTraversable(bool? value) => super.noSuchMethod(
        Invocation.setter(
          #descendantsAreTraversable,
          value,
        ),
        returnValueForMissingStub: null,
      );

  @override
  Iterable<_i3.FocusNode> get children => (super.noSuchMethod(
        Invocation.getter(#children),
        returnValue: <_i3.FocusNode>[],
        returnValueForMissingStub: <_i3.FocusNode>[],
      ) as Iterable<_i3.FocusNode>);

  @override
  Iterable<_i3.FocusNode> get traversalChildren => (super.noSuchMethod(
        Invocation.getter(#traversalChildren),
        returnValue: <_i3.FocusNode>[],
        returnValueForMissingStub: <_i3.FocusNode>[],
      ) as Iterable<_i3.FocusNode>);

  @override
  set debugLabel(String? value) => super.noSuchMethod(
        Invocation.setter(
          #debugLabel,
          value,
        ),
        returnValueForMissingStub: null,
      );

  @override
  Iterable<_i3.FocusNode> get descendants => (super.noSuchMethod(
        Invocation.getter(#descendants),
        returnValue: <_i3.FocusNode>[],
        returnValueForMissingStub: <_i3.FocusNode>[],
      ) as Iterable<_i3.FocusNode>);

  @override
  Iterable<_i3.FocusNode> get traversalDescendants => (super.noSuchMethod(
        Invocation.getter(#traversalDescendants),
        returnValue: <_i3.FocusNode>[],
        returnValueForMissingStub: <_i3.FocusNode>[],
      ) as Iterable<_i3.FocusNode>);

  @override
  Iterable<_i3.FocusNode> get ancestors => (super.noSuchMethod(
        Invocation.getter(#ancestors),
        returnValue: <_i3.FocusNode>[],
        returnValueForMissingStub: <_i3.FocusNode>[],
      ) as Iterable<_i3.FocusNode>);

  @override
  bool get hasFocus => (super.noSuchMethod(
        Invocation.getter(#hasFocus),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  bool get hasPrimaryFocus => (super.noSuchMethod(
        Invocation.getter(#hasPrimaryFocus),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  _i3.FocusHighlightMode get highlightMode => (super.noSuchMethod(
        Invocation.getter(#highlightMode),
        returnValue: _i3.FocusHighlightMode.touch,
        returnValueForMissingStub: _i3.FocusHighlightMode.touch,
      ) as _i3.FocusHighlightMode);

  @override
  _i2.Size get size => (super.noSuchMethod(
        Invocation.getter(#size),
        returnValue: _FakeSize_0(
          this,
          Invocation.getter(#size),
        ),
        returnValueForMissingStub: _FakeSize_0(
          this,
          Invocation.getter(#size),
        ),
      ) as _i2.Size);

  @override
  _i2.Offset get offset => (super.noSuchMethod(
        Invocation.getter(#offset),
        returnValue: _FakeOffset_1(
          this,
          Invocation.getter(#offset),
        ),
        returnValueForMissingStub: _FakeOffset_1(
          this,
          Invocation.getter(#offset),
        ),
      ) as _i2.Offset);

  @override
  _i2.Rect get rect => (super.noSuchMethod(
        Invocation.getter(#rect),
        returnValue: _FakeRect_2(
          this,
          Invocation.getter(#rect),
        ),
        returnValueForMissingStub: _FakeRect_2(
          this,
          Invocation.getter(#rect),
        ),
      ) as _i2.Rect);

  @override
  bool get hasListeners => (super.noSuchMethod(
        Invocation.getter(#hasListeners),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  void unfocus(
          {_i3.UnfocusDisposition? disposition =
              _i3.UnfocusDisposition.scope}) =>
      super.noSuchMethod(
        Invocation.method(
          #unfocus,
          [],
          {#disposition: disposition},
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool consumeKeyboardToken() => (super.noSuchMethod(
        Invocation.method(
          #consumeKeyboardToken,
          [],
        ),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  _i3.FocusAttachment attach(
    _i5.BuildContext? context, {
    _i3.FocusOnKeyEventCallback? onKeyEvent,
    _i3.FocusOnKeyCallback? onKey,
  }) =>
      (super.noSuchMethod(
        Invocation.method(
          #attach,
          [context],
          {
            #onKeyEvent: onKeyEvent,
            #onKey: onKey,
          },
        ),
        returnValue: _FakeFocusAttachment_3(
          this,
          Invocation.method(
            #attach,
            [context],
            {
              #onKeyEvent: onKeyEvent,
              #onKey: onKey,
            },
          ),
        ),
        returnValueForMissingStub: _FakeFocusAttachment_3(
          this,
          Invocation.method(
            #attach,
            [context],
            {
              #onKeyEvent: onKeyEvent,
              #onKey: onKey,
            },
          ),
        ),
      ) as _i3.FocusAttachment);

  @override
  void dispose() => super.noSuchMethod(
        Invocation.method(
          #dispose,
          [],
        ),
        returnValueForMissingStub: null,
      );

  @override
  void requestFocus([_i3.FocusNode? node]) => super.noSuchMethod(
        Invocation.method(
          #requestFocus,
          [node],
        ),
        returnValueForMissingStub: null,
      );

  @override
  bool nextFocus() => (super.noSuchMethod(
        Invocation.method(
          #nextFocus,
          [],
        ),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  bool previousFocus() => (super.noSuchMethod(
        Invocation.method(
          #previousFocus,
          [],
        ),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  bool focusInDirection(_i6.TraversalDirection? direction) =>
      (super.noSuchMethod(
        Invocation.method(
          #focusInDirection,
          [direction],
        ),
        returnValue: false,
        returnValueForMissingStub: false,
      ) as bool);

  @override
  void debugFillProperties(_i4.DiagnosticPropertiesBuilder? properties) =>
      super.noSuchMethod(
        Invocation.method(
          #debugFillProperties,
          [properties],
        ),
        returnValueForMissingStub: null,
      );

  @override
  List<_i4.DiagnosticsNode> debugDescribeChildren() => (super.noSuchMethod(
        Invocation.method(
          #debugDescribeChildren,
          [],
        ),
        returnValue: <_i4.DiagnosticsNode>[],
        returnValueForMissingStub: <_i4.DiagnosticsNode>[],
      ) as List<_i4.DiagnosticsNode>);

  @override
  String toStringShort() => (super.noSuchMethod(
        Invocation.method(
          #toStringShort,
          [],
        ),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  String toString({_i4.DiagnosticLevel? minLevel = _i4.DiagnosticLevel.info}) =>
      super.toString();

  @override
  String toStringShallow({
    String? joiner = r', ',
    _i4.DiagnosticLevel? minLevel = _i4.DiagnosticLevel.debug,
  }) =>
      (super.noSuchMethod(
        Invocation.method(
          #toStringShallow,
          [],
          {
            #joiner: joiner,
            #minLevel: minLevel,
          },
        ),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  String toStringDeep({
    String? prefixLineOne = r'',
    String? prefixOtherLines,
    _i4.DiagnosticLevel? minLevel = _i4.DiagnosticLevel.debug,
  }) =>
      (super.noSuchMethod(
        Invocation.method(
          #toStringDeep,
          [],
          {
            #prefixLineOne: prefixLineOne,
            #prefixOtherLines: prefixOtherLines,
            #minLevel: minLevel,
          },
        ),
        returnValue: '',
        returnValueForMissingStub: '',
      ) as String);

  @override
  _i4.DiagnosticsNode toDiagnosticsNode({
    String? name,
    _i4.DiagnosticsTreeStyle? style,
  }) =>
      (super.noSuchMethod(
        Invocation.method(
          #toDiagnosticsNode,
          [],
          {
            #name: name,
            #style: style,
          },
        ),
        returnValue: _FakeDiagnosticsNode_4(
          this,
          Invocation.method(
            #toDiagnosticsNode,
            [],
            {
              #name: name,
              #style: style,
            },
          ),
        ),
        returnValueForMissingStub: _FakeDiagnosticsNode_4(
          this,
          Invocation.method(
            #toDiagnosticsNode,
            [],
            {
              #name: name,
              #style: style,
            },
          ),
        ),
      ) as _i4.DiagnosticsNode);

  @override
  void addListener(_i2.VoidCallback? listener) => super.noSuchMethod(
        Invocation.method(
          #addListener,
          [listener],
        ),
        returnValueForMissingStub: null,
      );

  @override
  void removeListener(_i2.VoidCallback? listener) => super.noSuchMethod(
        Invocation.method(
          #removeListener,
          [listener],
        ),
        returnValueForMissingStub: null,
      );

  @override
  void notifyListeners() => super.noSuchMethod(
        Invocation.method(
          #notifyListeners,
          [],
        ),
        returnValueForMissingStub: null,
      );
}
