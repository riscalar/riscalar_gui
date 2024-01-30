import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/sim_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/system_configuration.dart';
import 'package:riscalar_gui/features/latency_simulation/results/results_page.dart';

class LatencySimulationPage extends ConsumerStatefulWidget {
  const LatencySimulationPage({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _LatencySimulationPageState();
}

class _LatencySimulationPageState extends ConsumerState<LatencySimulationPage> {
  @override
  Widget build(BuildContext context) {
    final simulationFileAvailable = ref
        .watch(configurationStateProvider.select((state) => state.binaryPath));
    final binaryChosen = simulationFileAvailable != '';
    return MacosScaffold(
      toolBar: ToolBar(
        title: const Text('Performance Simulation'),
        titleWidth: 180,
        leading: MacosTooltip(
          message: 'Toggle Sidebar',
          useMousePosition: false,
          child: MacosIconButton(
            icon: MacosIcon(
              CupertinoIcons.sidebar_left,
              color: MacosTheme.brightnessOf(context).resolve(
                const Color.fromRGBO(0, 0, 0, 0.5),
                const Color.fromRGBO(255, 255, 255, 0.5),
              ),
              size: 20,
            ),
            boxConstraints: const BoxConstraints(
              minHeight: 20,
              minWidth: 20,
              maxWidth: 48,
              maxHeight: 38,
            ),
            onPressed: () => MacosWindowScope.of(context).toggleSidebar(),
          ),
        ),
        actions: [
          ToolBarIconButton(
            icon: const MacosIcon(
              CupertinoIcons.settings,
            ),
            onPressed: () => debugPrint('New Folder...'),
            label: 'Save Configuration',
            showLabel: true,
            tooltipMessage: 'Save Current System Configuration to File',
          ),
          if (binaryChosen)
            ToolBarIconButton(
              label: 'Start Simulation',
              onPressed: () => Navigator.of(context).push(
                MaterialPageRoute(
                  builder: (_) => const PerformanceSimulationResults(),
                ),
              ),
              icon: MacosIcon(
                CupertinoIcons.play_circle_fill,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.systemGreenColor,
                  context,
                ),
              ),
              showLabel: true,
            )
        ],
      ),
      children: [
        ContentArea(
          builder: (context, _) => const SystemConfiguration(),
        ),
        ResizablePane(
          minSize: 380,
          startSize: 380,
          windowBreakpoint: 700,
          resizableSide: ResizableSide.left,
          builder: (context, scrollcontroller) {
            return const PerformanceSimulationSettingsPanel();
          },
        ),
      ],
    );
  }
}

const stats = [
  '',
  'IPC',
  'CPI',
  'Committed Instructions',
  'Executed Instructions',
  'LSQ Entries',
  'RSQ Entries',
  'L1I Hits',
  'L1I Misses',
  'L1D Hits',
  'L1D Misses',
  'L1U Hits',
  'L1U Misses',
  'L2I Hits',
  'L2I Misses',
  'L2D Hits',
  'L2D Misses',
];
