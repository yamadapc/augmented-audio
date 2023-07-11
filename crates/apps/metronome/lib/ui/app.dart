import 'dart:ui';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:metronome/modules/analytics/analytics.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/ui/home_page.dart';
import 'package:mobx/mobx.dart';

Observable<Brightness?> brightness =
    Observable(PlatformDispatcher.instance.platformBrightness);

class App extends StatefulWidget {
  final Analytics analytics;

  const App({
    super.key,
    required this.analytics,
  });

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> with WidgetsBindingObserver {
  @override
  void initState() {
    WidgetsBinding.instance.addObserver(this);

    super.initState();
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangePlatformBrightness() {
    runInAction(() {
      brightness.value = PlatformDispatcher.instance.platformBrightness;
    });
  }

  @override
  Widget build(BuildContext context) {
    return PlatformProvider(
      builder: (BuildContext context) {
        return AppContext(
          analytics: widget.analytics,
          child: Observer(
            builder: (context) => PlatformWidget(
              cupertino: (_, __) => MacosApp(
                title: 'Metronome',
                home: const HomePage(title: 'Metronome'),
                debugShowCheckedModeBanner: false,
                theme: brightness.value == Brightness.light
                    ? buildMacosThemeData(context)
                    : buildMacosThemeDataDark(context),
                darkTheme: buildMacosThemeDataDark(context),
              ),
              material: (_, __) => const PlatformApp(
                title: 'Metronome',
                home: HomePage(title: 'Metronome'),
                debugShowCheckedModeBanner: false,
              ),
            ),
          ),
        );
      },
    );
  }

  MacosThemeData buildMacosThemeDataDark(BuildContext context) {
    return MacosThemeData(
      brightness: Brightness.dark,
      typography: MacosTypography(
        color: Colors.black,
        body: const TextStyle(fontSize: 14, color: Colors.white),
      ),
      primaryColor: CupertinoColors.activeBlue.resolveFrom(context),
      canvasColor: CupertinoColors.activeBlue.resolveFrom(context),
    );
  }

  MacosThemeData buildMacosThemeData(BuildContext context) {
    return MacosThemeData(
      brightness: Brightness.light,
      typography: MacosTypography(
        color: Colors.black,
        body: const TextStyle(fontSize: 14, color: Colors.black),
      ),
      primaryColor: CupertinoColors.activeBlue.resolveFrom(context),
      canvasColor: CupertinoColors.activeBlue.resolveFrom(context),
    );
  }
}
