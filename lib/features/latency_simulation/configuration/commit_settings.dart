import 'package:flutter/cupertino.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:riscalar_gui/features/latency_simulation/configuration/state.dart';

class CommitSettings extends ConsumerStatefulWidget {
  const CommitSettings({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _CommitSettingsState();
}

class _CommitSettingsState extends ConsumerState<CommitSettings> {
  @override
  Widget build(BuildContext context) {
    final commitWidth = ref.watch(
      configurationStateProvider.select((cfg) => cfg.commitConfig.commitWidth),
    )!;
    return Padding(
      padding: const EdgeInsets.all(20),
      child: Column(
        children: [
          CupertinoListSection.insetGrouped(
            backgroundColor: MacosColors.transparent,
            header: const Text('Commit Unit'),
            children: [
              CupertinoListTile(
                title: const Text('Commit Width (Instrs/Cycle))'),
                padding: const EdgeInsets.all(15),
                subtitle: const Text(
                  'Number of instructions committed per cycle',
                ),
                trailing: MacosPopupButton<int>(
                  value: commitWidth,
                  menuMaxHeight: 240,
                  onChanged: (int? newValue) => ref
                      .read(configurationStateProvider.notifier)
                      .updateCommitWidth(newValue!),
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
        ],
      ),
    );
  }
}
