import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';
import 'package:riscalar_gui/src/rust/cpu/latency_core/latency_args.dart';

class IssueSettings extends ConsumerStatefulWidget {
  const IssueSettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _IssueSettingsState();
}

class _IssueSettingsState extends ConsumerState<IssueSettings> {
  late final MacosTabController _controller;

  @override
  void initState() {
    super.initState();
    _controller = MacosTabController(
      length: 2,
      initialIndex:
          ref.read(configurationStateProvider).issueConfig.issueOrder ==
                  IssueOrder.inOrder
              ? 0
              : 1,
    );
  }

  @override
  Widget build(BuildContext context) {
    final issueWidth = ref.watch(
      configurationStateProvider.select((cfg) => cfg.issueConfig.issueWidth),
    );
    final noMissSpec = ref.watch(
      configurationStateProvider
          .select((cfg) => cfg.issueConfig.issueNoMisspec),
    );

    _controller.addListener(() {
      ref.read(configurationStateProvider.notifier).updateIssueOrder(
            _controller.index == 0 ? IssueOrder.inOrder : IssueOrder.outOrder,
          );
    });

    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('Issue Unit'),
            children: [
              CupertinoListTile(
                title: const Text('Issue Width (Instrs/Cycle))'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Number of instructions issued per cycle',
                ),
                trailing: MacosPopupButton<int>(
                  value: issueWidth,
                  menuMaxHeight: 200,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateIssueWidth(newValue!),
                  items: <int>[1, 2, 4, 8, 16, 32, 64, 128]
                      .map<MacosPopupMenuItem<int>>((int value) {
                    return MacosPopupMenuItem<int>(
                      value: value,
                      child: Text(value.toString()),
                    );
                  }).toList(),
                ),
              ),
              CupertinoListTile(
                title: const Text('Issue Order'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                    'Allow later instructions with satisfied dependencies '
                    'to be issued earlier'),
                trailing: MacosSegmentedControl(
                  controller: _controller,
                  tabs: [
                    MacosTab(
                      label: 'In Order',
                      active: _controller.index == 0,
                    ),
                    MacosTab(
                      label: 'Out Of Order',
                      active: _controller.index == 1,
                    ),
                  ],
                ),
              ),
              CupertinoListTile(
                padding: const EdgeInsets.all(15),
                title: const Text('No Mis-Speculation'),
                subtitle: const Text(' Prevent instructions from being issued '
                    'down a mispredicted speculative branch'),
                trailing: MacosSwitch(
                  activeColor: MacosColors.appleGreen,
                  value: noMissSpec,
                  onChanged: (value) => ref
                      .read(configurationStateProvider.notifier)
                      .updateIssueNoMisspec(noMisspec: value),
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
