import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/functional_simulation/page.dart';
import 'package:riscalar_gui/features/latency_simulation/page.dart';

void main() {
  runApp(const ProviderScope(child: Riscalar()));
}

/// Top level app widget
class Riscalar extends ConsumerWidget {
  /// Creates a new [Riscalar]
  const Riscalar({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MacosApp(
      title: 'riscalar',
      theme: MacosThemeData.light(),
      darkTheme: MacosThemeData.dark(),
      themeMode: ThemeMode.system,
      home: const HomePage(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});
  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int pageIndex = 0;
  final List<Widget> pages = const [
    LatencySimulationPage(),
    FunctionalSimulationPage()
  ];
  @override
  Widget build(BuildContext context) {
    return MacosWindow(
      sidebar: Sidebar(
        startWidth: 180,
        minWidth: 180,
        maxWidth: 350,
        builder: (context, scrollController) => SidebarItems(
          currentIndex: pageIndex,
          onChanged: (i) => setState(() => pageIndex = i),
          scrollController: scrollController,
          itemSize: SidebarItemSize.large,
          items: const [
            SidebarItem(
              // leading: MacosIcon(CupertinoIcons.square_on_circle),
              leading: MacosIcon(
                CupertinoIcons.bolt_fill,
                color: Color(0xFFFECE0E),
              ),
              label: Text('Performance'),
            ),
            SidebarItem(
              // leading: MacosIcon(CupertinoIcons.square_on_circle),
              leading: MacosIcon(
                CupertinoIcons.flowchart,
                color: MacosColors.systemTealColor,
              ),
              label: Text('Functional'),
            ),
          ],
        ),
        bottom: MacosListTile(
          title: const Text('Riscalar'),
          subtitle: const Text('By Josiah Mendes'),
          // ignore: inference_failure_on_function_invocation
          onClick: () => showMacosSheet(
            context: context,
            builder: (context) => MacosAlertDialog(
              appIcon: Image.asset(
                'assets/icon/icon.png',
                width: 56,
                height: 56,
              ),
              title: const Text('About Riscalar'),
              message: const Text(
                'Riscalar is a RISC-V simulator written in Rust. '
                'It is designed with education in mind, and can do both '
                'functional simulation and performance simulation. '
                'Its performance simulator is based on SimpleScalar, '
                'and can simulate superscalar architectures '
                'with multiple cache levels, pipeline widths, branch '
                'prediction, and more. ',
                textAlign: TextAlign.justify,
              ),
              //horizontalActions: false,
              primaryButton: PushButton(
                onPressed: Navigator.of(context).pop,
                controlSize: ControlSize.large,
                child: const Text('Dismiss'),
              ),
            ),
          ),
        ),
      ),
      child: IndexedStack(
        index: pageIndex,
        children: pages,
      ),
    );
  }
}
