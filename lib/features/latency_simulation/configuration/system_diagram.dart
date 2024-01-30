import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:widget_arrows/widget_arrows.dart';

class SystemDiagram extends StatelessWidget {
  const SystemDiagram({
    required MacosTabController controller,
    super.key,
  }) : _controller = controller;

  final MacosTabController _controller;

  @override
  Widget build(BuildContext context) {
    return ArrowContainer(
      child: Column(
        children: [
          Row(
            children: [
              ArrowElement(
                id: 'branch_predictor',
                targetIds: const ['fetch_stage'],
                sourceAnchor: Alignment.bottomCenter,
                targetAnchor: Alignment.topCenter,
                doubleSided: true,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemOrangeColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Branch Predictor',
                  onPressed: () => _controller.index = 0,
                ),
              ),
              const Spacer(flex: 3),
              ArrowElement(
                id: 'cache',
                targetIds: const ['fetch_stage'],
                targetAnchor: Alignment.topCenter,
                bow: 0.1,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemOrangeColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Cache Hierarchy',
                  onPressed: () => _controller.index = 5,
                ),
              ),
              const Spacer(flex: 3),
              ArrowElement(
                id: 'main_memory',
                targetAnchor: Alignment.centerRight,
                doubleSided: true,
                targetId: 'cache',
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemOrangeColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Main Memory',
                  onPressed: () => _controller.index = 5,
                ),
              ),
            ],
          ),
          const SizedBox(height: 80),
          Row(
            children: [
              ArrowElement(
                id: 'fetch_stage',
                targetId: 'dispatch_stage',
                sourceAnchor: Alignment.centerRight,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Fetch\nStage',
                  onPressed: () => _controller.index = 0,
                ),
              ),
              const Spacer(),
              ArrowElement(
                id: 'dispatch_stage',
                targetId: 'issue_stage',
                sourceAnchor: Alignment.centerRight,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Dispatch\nStage',
                  onPressed: () => _controller.index = 1,
                ),
              ),
              const Spacer(),
              ArrowElement(
                id: 'issue_stage',
                targetId: 'execute_stage',
                sourceAnchor: Alignment.centerRight,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Issue\nStage',
                  onPressed: () => _controller.index = 2,
                ),
              ),
              const Spacer(),
              ArrowElement(
                id: 'execute_stage',
                targetId: 'write_back_stage',
                sourceAnchor: Alignment.centerRight,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Execute\nStage',
                  onPressed: () => _controller.index = 3,
                ),
              ),
              const Spacer(),
              ArrowElement(
                id: 'write_back_stage',
                targetId: 'commit_stage',
                sourceAnchor: Alignment.centerRight,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Write-Back\nStage',
                ),
              ),
              const Spacer(),
              ArrowElement(
                id: 'commit_stage',
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemGreenColor,
                    context,
                  ),
                  flex: 8,
                  text: 'Commit\nStage',
                  onPressed: () => _controller.index = 4,
                ),
              ),
            ],
          ),
          const SizedBox(height: 80),
          Row(
            children: [
              const Spacer(),
              Component(
                color: CupertinoDynamicColor.resolve(
                  MacosColors.systemOrangeColor,
                  context,
                ),
                flex: 8,
                text: 'Load Store Queue',
                onPressed: () => _controller.index = 1,
              ),
              const Spacer(),
              Component(
                color: CupertinoDynamicColor.resolve(
                  MacosColors.systemOrangeColor,
                  context,
                ),
                flex: 14,
                text: 'Reservation Station Queue',
                onPressed: () => _controller.index = 1,
              ),
              const Spacer(flex: 3),
              ArrowElement(
                id: 'functional_unit_pool',
                targetIds: const ['execute_stage', 'write_back_stage'],
                doubleSided: true,
                sourceAnchor: Alignment.topCenter,
                targetAnchor: Alignment.bottomCenter,
                color: CupertinoDynamicColor.resolve(
                  MacosColors.labelColor,
                  context,
                ),
                flip: true,
                stretch: 0,
                child: Component(
                  color: CupertinoDynamicColor.resolve(
                    MacosColors.systemOrangeColor,
                    context,
                  ),
                  flex: 5,
                  text: 'Functional Unit Pool',
                  onPressed: () => _controller.index = 3,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}

class Component extends StatelessWidget {
  const Component({
    required this.color,
    required this.text,
    required this.flex,
    this.onPressed,
    super.key,
  });
  final Color color;
  final String text;
  final int flex;
  final Function()? onPressed;

  @override
  Widget build(BuildContext context) {
    return Flexible(
      flex: flex,
      child: PushButton(
        controlSize: ControlSize.large,
        borderRadius: BorderRadius.circular(20),
        color: color,
        onPressed: onPressed,
        child: Container(
          padding: const EdgeInsets.all(8),
          height: 100,
          width: 150,
          alignment: Alignment.center,
          child: Text(
            text,
            style: MacosTheme.of(context).typography.title1,
            textAlign: TextAlign.center,
          ),
        ),
      ),
    );
  }
}
