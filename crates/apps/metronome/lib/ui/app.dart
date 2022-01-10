import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

import 'home_page.dart';

Observable<Brightness?> brightness =
    Observable(WidgetsBinding.instance?.window.platformBrightness);

class App extends StatefulWidget {
  const App({Key? key}) : super(key: key);

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> with WidgetsBindingObserver {
  @override
  void initState() {
    WidgetsBinding.instance?.addObserver(this);
    super.initState();
  }

  @override
  void dispose() {
    WidgetsBinding.instance?.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangePlatformBrightness() {
    runInAction(() {
      brightness.value = WidgetsBinding.instance?.window.platformBrightness;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => CupertinoApp(
        title: 'Metronome',
        theme:
            CupertinoThemeData(brightness: brightness.value ?? Brightness.dark),
        home: const HomePage(title: 'Metronome'),
      ),
    );
  }
}
