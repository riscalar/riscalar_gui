import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';

class DispatchSettings extends ConsumerStatefulWidget {
  const DispatchSettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _DispatchSettingsState();
}

class _DispatchSettingsState extends ConsumerState<DispatchSettings> {
  // int lsqSize = 8;
  // int rsqSize = 16;
  // int decodeWidth = 4;
  @override
  Widget build(BuildContext context) {
    final lsqSize =
        ref.watch(configurationStateProvider.select((cfg) => cfg.lsqSize));
    final rsqSize =
        ref.watch(configurationStateProvider.select((cfg) => cfg.rsqSize));
    final decodeWidth = ref.watch(
      configurationStateProvider.select((cfg) => cfg.decodeConfig.decodeWidth),
    );
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('Dispatch Unit'),
            children: [
              CupertinoListTile(
                title: const Text('Decode Width (Instrs/Cycle))'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Number of instructions decoded per cycle',
                ),
                trailing: MacosPopupButton<int>(
                  value: decodeWidth,
                  menuMaxHeight: 250,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateDecodeWidth(newValue!),
                  items: <int>[1, 2, 4, 8, 16, 32, 64, 128]
                      .map<MacosPopupMenuItem<int>>((int value) {
                    return MacosPopupMenuItem<int>(
                      value: value,
                      child: Text(value.toString()),
                    );
                  }).toList(),
                ),
              ),
            ],
          ),
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('In-Flight Instruction Queues'),
            children: [
              CupertinoListTile(
                title: const Text('Reservation Station Queue Size'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Maximum non-load/stores instrs allowed in-flight at given time',
                ),
                trailing: MacosPopupButton<int>(
                  value: rsqSize,
                  menuMaxHeight: 250,
                  onChanged: (int? newValue) => ref
                      .read(
                        configurationStateProvider.notifier,
                      )
                      .updateRsqSize(newValue!),
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
                title: const Text('Load Store Queue Size'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Maximum load/stores instrs allowed in-flight at given time',
                ),
                trailing: MacosPopupButton<int>(
                  value: lsqSize,
                  menuMaxHeight: 250,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateLsqSize(newValue!),
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
            ],
          ),
        ],
      ),
    );
  }
}
