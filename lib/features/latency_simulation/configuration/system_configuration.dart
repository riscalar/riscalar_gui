import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/commit_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/dispatch_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/execute_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/fetch_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/issue_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/memory_settings.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/system_diagram.dart';

class SystemConfiguration extends StatefulWidget {
  const SystemConfiguration({
    super.key,
  });

  @override
  State<SystemConfiguration> createState() => _SystemConfigurationState();
}

class _SystemConfigurationState extends State<SystemConfiguration> {
  final _controller = MacosTabController(length: 6);
  // final viewTransformationController = TransformationController();

  // @override
  // void initState() {
  //   const zoomFactor = 0.1;
  //   const xTranslate = -20.0;
  //   const yTranslate = -50.0;
  //   viewTransformationController.value.setEntry(0, 0, zoomFactor);
  //   viewTransformationController.value.setEntry(1, 1, zoomFactor);
  //   viewTransformationController.value.setEntry(2, 2, zoomFactor);
  //   viewTransformationController.value.setEntry(0, 3, -xTranslate);
  //   viewTransformationController.value.setEntry(1, 3, -yTranslate);
  //   super.initState();
  // }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Flexible(
          child: Container(
            width: double.infinity,
            alignment: Alignment.center,
            child: InteractiveViewer(
              // transformationController: viewTransformationController,
              constrained: false,
              boundaryMargin: const EdgeInsets.all(double.infinity),
              minScale: 0.1,
              maxScale: 1.6,
              child: SizedBox(
                height: 800,
                width: 2800,
                child: SystemDiagram(controller: _controller),
              ),
            ),
          ),
        ),
        ResizablePane(
          minSize: 100,
          startSize: 350,
          windowBreakpoint: 700,
          resizableSide: ResizableSide.top,
          builder: (context, scrollcontroller) {
            return SingleChildScrollView(
              controller: scrollcontroller,
              child: Padding(
                padding: const EdgeInsets.all(20),
                child: MacosTabView(
                  controller: _controller,
                  tabs: const [
                    MacosTab(label: 'Fetch'),
                    MacosTab(label: 'Dispatch'),
                    MacosTab(label: 'Issue'),
                    MacosTab(label: 'Execute'),
                    MacosTab(label: 'Commit'),
                    MacosTab(label: 'Memory'),
                  ],
                  children: const [
                    FetchSettings(),
                    DispatchSettings(),
                    IssueSettings(),
                    ExecuteSettings(),
                    CommitSettings(),
                    MemorySettings(),
                  ],
                ),
              ),
            );
          },
        ),
      ],
    );
  }
}
