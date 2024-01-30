import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/configuration_table.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';
import 'package:riscalar_gui/features/latency_simulation/results/register_pane.dart';
import 'package:riscalar_gui/src/rust/api/simple.dart';

import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'results_page.g.dart';

/// Uses Rust bridge to run a simulation
@riverpod
Future<SimulationResult?> simulate(SimulateRef ref) async {
  final result = await runSimulationLatency(
    args: ref.watch(configurationStateProvider),
  );
  return result;
}

class PerformanceSimulationResults extends ConsumerWidget {
  const PerformanceSimulationResults({
    super.key,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final simulationResult = ref.watch(simulateProvider);

    return MacosScaffold(
      toolBar: ToolBar(
        title: const Text('Simulation Results'),
        actions: [
          ToolBarIconButton(
            icon: const MacosIcon(
              CupertinoIcons.arrow_down_doc_fill,
            ),
            onPressed: () => debugPrint('New Folder...'),
            label: 'Save Results',
            showLabel: true,
            tooltipMessage: 'Save System Simulation Results to File',
          ),
        ],
      ),
      children: simulationResult.when(
        loading: () => <Widget>[
          ContentArea(
            builder: (context, _) {
              return Center(
                child: Container(
                  // Max width = 1/3 of the screen
                  constraints: BoxConstraints(
                    maxWidth: MediaQuery.of(context).size.width / 3,
                  ),
                  padding: const EdgeInsets.all(20),
                  child: const Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text('Simulation in progress...'),
                      SizedBox(height: 20),
                      LinearProgressIndicator(),
                    ],
                  ),
                ),
              );
            },
          ),
        ],
        error: (error, stackTrace) => <Widget>[
          ContentArea(
            builder: (context, _) => Center(child: Text('Error: $error')),
          ),
        ],
        data: (result) => <Widget>[
          ContentArea(
            builder: (context, _) => result!.when(
              success: (_, field0, field1) => Center(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    ConstrainedBox(
                      constraints: const BoxConstraints(minHeight: 600),
                      child: Container(
                        alignment: Alignment.center,
                        child: Text('Success: $field0'),
                      ),
                    ),
                    ResizablePane(
                      minSize: 100,
                      startSize: 350,
                      windowBreakpoint: 700,
                      resizableSide: ResizableSide.top,
                      builder: (context, controller) =>
                          LatencyConfigurationTable(
                        scrollController: controller,
                      ),
                    ),
                  ],
                ),
              ),
              error: (field0, field1) => Center(child: Text('Error: $field0')),
            ),
          ),
          ResizablePane(
            minSize: 280,
            startSize: 380,
            windowBreakpoint: 700,
            resizableSide: ResizableSide.left,
            builder: (context, scrollController) => result!.when(
              success: (reg, s1, s2) => ResultsRegisterPane(
                reg,
                scrollController: scrollController,
              ),
              error: (field0, field1) => Container(),
            ),
          ),
        ],
      ),
    );
  }
}
