import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';

class FetchSettings extends ConsumerStatefulWidget {
  const FetchSettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _FetchSettingsState();
}

class _FetchSettingsState extends ConsumerState<FetchSettings> {
  // int fetchDispatchQueueSize = 4;
  // int fetchSpeed = 1;
  final _controller = MacosTabController(length: 3);
  final branchPenaltyTextController = TextEditingController(text: '3');

  @override
  void initState() {
    super.initState();
    branchPenaltyTextController.addListener(() {
      final value = int.tryParse(branchPenaltyTextController.text);
      if (value != null) {
        ref
            .read(configurationStateProvider.notifier)
            .updateFetchBranchPenalty(value);
      }
    });
  }

  @override
  void dispose() {
    branchPenaltyTextController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final fetchSpeed = ref.watch(
      configurationStateProvider.select((cfg) => cfg.fetchConfig.fetchSpeed),
    );
    final fetchDispatchQueueSize = ref.watch(
      configurationStateProvider
          .select((cfg) => cfg.fetchConfig.fetchQueueSize),
    );

    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            header: const Text('Fetch Unit'),
            backgroundColor: MacosColors.transparent,
            children: [
              CupertinoListTile(
                title: const Text('Fetch Dispatch Queue Size'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Maximum instructions between fetch and dispatch stages',
                ),
                trailing: MacosPopupButton<int>(
                  value: fetchDispatchQueueSize,
                  menuMaxHeight: 200,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateFetchDispatchQueueSize(newValue!),
                  items: <int>[
                    2,
                    4,
                    8,
                    16,
                    32,
                    64,
                    128,
                    256,
                    512,
                    1024,
                    2048,
                    4096,
                    8192,
                    16384,
                    32768
                  ].map<MacosPopupMenuItem<int>>((int value) {
                    return MacosPopupMenuItem<int>(
                      value: value,
                      child: Text(value.toString()),
                    );
                  }).toList(),
                ),
              ),
              CupertinoListTile(
                title: const Text('Fetch Speed'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Speed of Fetch Unit relative to Decode/Dispatch Unit',
                ),
                trailing: MacosPopupButton<int>(
                  value: fetchSpeed,
                  menuMaxHeight: 200,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateFetchSpeed(newValue!),
                  items: <int>[1, 2, 3, 4, 5, 6, 7, 8]
                      .map<MacosPopupMenuItem<int>>((int value) {
                    return MacosPopupMenuItem<int>(
                      value: value,
                      child: Text(value.toString()),
                    );
                  }).toList(),
                ),
              ),
              CupertinoListTile(
                title: const Text('Fetch Branch Penalty'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text('Penalty for a mispredicted branch'),
                trailing: SizedBox(
                  width: 300,
                  child: Focus(
                    onFocusChange: (hasFocus) => {
                      if (!hasFocus && branchPenaltyTextController.text.isEmpty)
                        {
                          branchPenaltyTextController.text = '3',
                        }
                    },
                    child: MacosTextField(
                      textAlign: TextAlign.end,
                      controller: branchPenaltyTextController,
                      decoration: const BoxDecoration(
                        color: MacosColors.transparent,
                      ),
                      inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                      onChanged: (value) {
                        if (value.isNotEmpty) {
                          ref
                              .read(configurationStateProvider.notifier)
                              .updateFetchBranchPenalty(int.parse(value));
                        }
                      },
                      placeholder: 'Enter a Number',
                    ),
                  ),
                ),
              ),
            ],
          ),
          CupertinoListSection.insetGrouped(
            header: const Text('Branch Predictor'),
            backgroundColor: MacosColors.transparent,
            children: [
              CupertinoListTile(
                title: const Text('Branch Predictor Type'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text('Penalty for a mispredicted branch'),
                trailing: MacosSegmentedControl(
                  controller: _controller,
                  tabs: [
                    MacosTab(
                      label: 'Perfect',
                      active: _controller.index == 0,
                    ),
                    MacosTab(
                      label: 'Taken',
                      active: _controller.index == 1,
                    ),
                    MacosTab(
                      label: 'Not-Taken',
                      active: _controller.index == 2,
                    )
                  ],
                ),
              ),
            ],
          )
        ],
      ),
    );
  }
}
